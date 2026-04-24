// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
use crate::models::services::{
    ServiceChange, ServiceEntry, ServicePreset, ServicePresetResult, ServiceStartType, ServiceState,
};
use crate::util::silent_cmd;
use std::collections::HashMap;
use tauri::Emitter;
use winreg::enums::*;
use winreg::RegKey;
use wmi::WMIConnection;

/// Services that must never be disabled — disabling them can render Windows
/// unbootable or break fundamental subsystems (RPC, event log, power, etc.).
const NEVER_DISABLE: &[&str] = &[
    "RpcSs",
    "DcomLaunch",
    "Power",
    "PlugPlay",
    "CryptSvc",
    "EventLog",
    "Schedule",
    "Themes",
    "AudioSrv",
    "BFE",
    "MpsSvc",
    "Dhcp",
    "Dnscache",
    "WinDefend",
    "SecurityHealthService",
    "Winmgmt",
    "sppsvc",
    "BrokerInfrastructure",
    "CoreMessagingRegistrar",
    "WpnService",
    "LanmanWorkstation",
    "UserManager",
];

fn is_protected(name: &str) -> bool {
    NEVER_DISABLE.iter().any(|p| p.eq_ignore_ascii_case(name))
}

fn read_delayed_autostart(name: &str) -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey(format!(r"SYSTEM\CurrentControlSet\Services\{}", name))
        .ok()
        .and_then(|k| k.get_value::<u32, _>("DelayedAutostart").ok())
        .map(|v| v == 1)
        .unwrap_or(false)
}

fn map_start_mode(mode: &str, service_name: &str) -> ServiceStartType {
    match mode {
        "Auto" => {
            if read_delayed_autostart(service_name) {
                ServiceStartType::AutoDelayed
            } else {
                ServiceStartType::Automatic
            }
        }
        "Manual" => ServiceStartType::Manual,
        "Disabled" => ServiceStartType::Disabled,
        _ => ServiceStartType::Unknown,
    }
}

fn map_state(state: &str) -> ServiceState {
    match state {
        "Running" => ServiceState::Running,
        "Stopped" => ServiceState::Stopped,
        "Start Pending" => ServiceState::StartPending,
        "Stop Pending" => ServiceState::StopPending,
        _ => ServiceState::Unknown,
    }
}

fn start_type_to_arg(st: &ServiceStartType) -> Option<&'static str> {
    match st {
        ServiceStartType::Automatic => Some("Automatic"),
        ServiceStartType::AutoDelayed => Some("AutomaticDelayedStart"),
        ServiceStartType::Manual => Some("Manual"),
        ServiceStartType::Disabled => Some("Disabled"),
        ServiceStartType::Unknown => None,
    }
}

fn parse_start_type(s: &str) -> Result<ServiceStartType, String> {
    match s.to_ascii_lowercase().as_str() {
        "automatic" | "auto" => Ok(ServiceStartType::Automatic),
        "autodelayed" | "automaticdelayedstart" | "delayed" => Ok(ServiceStartType::AutoDelayed),
        "manual" => Ok(ServiceStartType::Manual),
        "disabled" => Ok(ServiceStartType::Disabled),
        other => Err(format!("Unknown start type: {}", other)),
    }
}

fn apply_start_type_sync(name: &str, start_type: &ServiceStartType) -> Result<(), String> {
    if is_protected(name) {
        return Err(format!(
            "'{}' is protected — disabling it can break Windows",
            name
        ));
    }
    let arg = start_type_to_arg(start_type)
        .ok_or_else(|| "Cannot apply Unknown start type".to_string())?;
    let script = format!(
        "Set-Service -Name '{}' -StartupType {} -ErrorAction Stop",
        name.replace('\'', "''"),
        arg
    );
    let out = silent_cmd("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| format!("run powershell: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "Set-Service failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }

    // If disabling, also stop any running instance so the change takes effect now.
    if matches!(start_type, ServiceStartType::Disabled) {
        let _ = silent_cmd("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Stop-Service -Name '{}' -Force -ErrorAction SilentlyContinue",
                    name.replace('\'', "''")
                ),
            ])
            .output();
    }
    Ok(())
}

#[tauri::command]
pub async fn get_services() -> Result<Vec<ServiceEntry>, String> {
    tokio::task::spawn_blocking(|| {
        let wmi = WMIConnection::new().map_err(|e| format!("WMI connect: {}", e))?;
        let results: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query("SELECT Name, DisplayName, Description, StartMode, State FROM Win32_Service")
            .map_err(|e| format!("WMI query: {}", e))?;

        let mut out = Vec::with_capacity(results.len());
        for row in results {
            let name = match row.get("Name") {
                Some(wmi::Variant::String(s)) => s.clone(),
                _ => continue,
            };
            let display_name = match row.get("DisplayName") {
                Some(wmi::Variant::String(s)) => s.clone(),
                _ => name.clone(),
            };
            let description = match row.get("Description") {
                Some(wmi::Variant::String(s)) => s.clone(),
                _ => String::new(),
            };
            let start_mode = match row.get("StartMode") {
                Some(wmi::Variant::String(s)) => s.clone(),
                _ => String::new(),
            };
            let state_raw = match row.get("State") {
                Some(wmi::Variant::String(s)) => s.clone(),
                _ => String::new(),
            };

            out.push(ServiceEntry {
                start_type: map_start_mode(&start_mode, &name),
                current_state: map_state(&state_raw),
                is_protected: is_protected(&name),
                name,
                display_name,
                description,
            });
        }
        out.sort_by(|a, b| {
            a.display_name
                .to_ascii_lowercase()
                .cmp(&b.display_name.to_ascii_lowercase())
        });
        Ok(out)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn set_service_start_type(name: String, start_type: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let parsed = parse_start_type(&start_type)?;
        apply_start_type_sync(&name, &parsed)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

fn change(name: &str, start_type: ServiceStartType, rationale: &str) -> ServiceChange {
    ServiceChange {
        service_name: name.to_string(),
        target_start_type: start_type,
        rationale: rationale.to_string(),
    }
}

fn build_presets() -> Vec<ServicePreset> {
    vec![
        ServicePreset {
            id: "gaming".to_string(),
            name: "Gaming".to_string(),
            description: "Disables telemetry and indexing services that compete with games for CPU and disk.".to_string(),
            changes: vec![
                change("DiagTrack", ServiceStartType::Disabled, "Connected User Experiences and Telemetry — reports to Microsoft"),
                change("SysMain", ServiceStartType::Disabled, "Superfetch — unnecessary on SSDs, can cause disk activity spikes"),
                change("WSearch", ServiceStartType::Disabled, "Windows Search indexing — noticeable disk churn during gaming"),
                change("dmwappushservice", ServiceStartType::Disabled, "Device Management WAP Push — part of telemetry stack"),
                change("MapsBroker", ServiceStartType::Disabled, "Downloaded Maps Manager — only needed for offline maps"),
                change("PcaSvc", ServiceStartType::Disabled, "Program Compatibility Assistant — background scanning"),
                change("RetailDemo", ServiceStartType::Disabled, "Retail Demo — store display mode, never used by end users"),
                change("Fax", ServiceStartType::Disabled, "Fax service — legacy"),
                change("WMPNetworkSvc", ServiceStartType::Disabled, "Windows Media Player Network Sharing — obsolete"),
            ],
        },
        ServicePreset {
            id: "privacy".to_string(),
            name: "Privacy".to_string(),
            description: "Stops services that report diagnostics, location, and usage back to Microsoft.".to_string(),
            changes: vec![
                change("DiagTrack", ServiceStartType::Disabled, "Telemetry — primary diagnostic data pipeline"),
                change("dmwappushservice", ServiceStartType::Disabled, "Device Management WAP Push — telemetry-adjacent"),
                change("lfsvc", ServiceStartType::Disabled, "Geolocation service — system-wide location tracking"),
                change("WerSvc", ServiceStartType::Disabled, "Windows Error Reporting — sends crash data to Microsoft"),
                change("PcaSvc", ServiceStartType::Disabled, "Program Compatibility Assistant — background scanning"),
                change("RetailDemo", ServiceStartType::Disabled, "Retail Demo — never used by end users"),
            ],
        },
        ServicePreset {
            id: "performance".to_string(),
            name: "Performance".to_string(),
            description: "Shuts down background services that consume CPU, RAM, and disk with little user benefit.".to_string(),
            changes: vec![
                change("DiagTrack", ServiceStartType::Disabled, "Telemetry — background CPU and network use"),
                change("SysMain", ServiceStartType::Disabled, "Superfetch — SSD-era systems gain little from it"),
                change("WSearch", ServiceStartType::Disabled, "Windows Search — heavy indexing load"),
                change("MapsBroker", ServiceStartType::Disabled, "Downloaded Maps Manager — background bandwidth"),
                change("WMPNetworkSvc", ServiceStartType::Disabled, "Windows Media Player Network Sharing — obsolete"),
                change("Fax", ServiceStartType::Disabled, "Fax service — legacy"),
            ],
        },
    ]
}

#[tauri::command]
pub async fn get_service_presets() -> Result<Vec<ServicePreset>, String> {
    Ok(build_presets())
}

#[tauri::command]
pub async fn apply_service_preset(
    app_handle: tauri::AppHandle,
    preset_id: String,
) -> Result<Vec<ServicePresetResult>, String> {
    // Best-effort restore point — don't fail the whole apply if System Restore is disabled.
    let _ = crate::commands::debloat::create_restore_point().await;

    let preset = build_presets()
        .into_iter()
        .find(|p| p.id == preset_id)
        .ok_or_else(|| format!("Unknown preset: {}", preset_id))?;

    tokio::task::spawn_blocking(move || {
        let mut results = Vec::with_capacity(preset.changes.len());
        for change in &preset.changes {
            let result = if is_protected(&change.service_name) {
                ServicePresetResult {
                    service_name: change.service_name.clone(),
                    success: false,
                    message: "Protected — skipped".to_string(),
                }
            } else {
                match apply_start_type_sync(&change.service_name, &change.target_start_type) {
                    Ok(()) => ServicePresetResult {
                        service_name: change.service_name.clone(),
                        success: true,
                        message: format!(
                            "Set to {}",
                            start_type_to_arg(&change.target_start_type).unwrap_or("Unknown")
                        ),
                    },
                    Err(e) => ServicePresetResult {
                        service_name: change.service_name.clone(),
                        success: false,
                        message: e,
                    },
                }
            };
            let _ = app_handle.emit("service-preset-progress", &result);
            results.push(result);
        }
        Ok(results)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}
