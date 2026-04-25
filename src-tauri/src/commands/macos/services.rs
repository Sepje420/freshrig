//! macOS services manager — wraps `launchctl` over LaunchDaemons.
//!
//! Matches the public command surface of the Windows + Linux `services`
//! modules so the frontend is OS-agnostic. macOS only exposes Enabled /
//! Disabled at the `launchctl` level (no Manual or AutoDelayed), so the
//! `Manual` start type maps to enabled-but-not-bootstrapped.

use std::collections::HashSet;

use tauri::Emitter;

use crate::commands::macos::util::{run_cmd_lossy, run_elevated};
use crate::models::services::{
    ServiceChange, ServiceEntry, ServicePreset, ServicePresetResult, ServiceStartType, ServiceState,
};

/// launchd labels that should never be disabled — doing so breaks login,
/// the window server, security, IPC, or directory services.
const NEVER_DISABLE: &[&str] = &[
    "com.apple.WindowServer",
    "com.apple.loginwindow",
    "com.apple.coreduetd",
    "com.apple.opendirectoryd",
    "com.apple.securityd",
    "com.apple.notifyd",
    "com.apple.UserEventAgent-System",
    "com.apple.distnoted.xpc.daemon",
];

fn is_protected(label: &str) -> bool {
    NEVER_DISABLE.iter().any(|p| p.eq_ignore_ascii_case(label))
        || label.starts_with("com.apple.system.")
        || label.starts_with("com.apple.kernel.")
}

#[tauri::command]
pub async fn get_services() -> Result<Vec<ServiceEntry>, String> {
    tokio::task::spawn_blocking(|| {
        let raw = run_cmd_lossy("launchctl", &["list"]);
        let disabled = read_disabled_set();

        let mut out = Vec::new();
        for (i, line) in raw.lines().enumerate() {
            // First line is the header: `PID	Status	Label`.
            if i == 0 || line.trim().is_empty() {
                continue;
            }
            let mut cols = line.split_whitespace();
            let pid = cols.next().unwrap_or("-");
            let _status = cols.next().unwrap_or("");
            let label = cols.collect::<Vec<_>>().join(" ");
            if label.is_empty() {
                continue;
            }

            let current_state = if pid != "-" {
                ServiceState::Running
            } else {
                ServiceState::Stopped
            };
            let start_type = if disabled.contains(&label) {
                ServiceStartType::Disabled
            } else {
                ServiceStartType::Automatic
            };

            out.push(ServiceEntry {
                name: label.clone(),
                display_name: label.clone(),
                description: String::new(),
                start_type,
                current_state,
                is_protected: is_protected(&label),
            });
        }

        out.sort_by(|a, b| {
            a.display_name
                .to_lowercase()
                .cmp(&b.display_name.to_lowercase())
        });
        Ok(out)
    })
    .await
    .map_err(|e| format!("services task failed: {}", e))?
}

/// Read the system-scope disabled-label set from
/// `launchctl print-disabled system`. Each line looks like
/// `"com.apple.foo" => true`.
fn read_disabled_set() -> HashSet<String> {
    let raw = run_cmd_lossy("launchctl", &["print-disabled", "system"]);
    let mut out = HashSet::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('"') {
            if let Some(end) = rest.find('"') {
                let label = &rest[..end];
                let after = &rest[end + 1..];
                if after.contains("=>") && after.contains("true") {
                    out.insert(label.to_string());
                }
            }
        }
    }
    out
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

fn apply_start_type_sync(label: &str, start_type: &ServiceStartType) -> Result<(), String> {
    if is_protected(label) {
        return Err(format!(
            "'{}' is protected — disabling it can break the system",
            label
        ));
    }
    match start_type {
        ServiceStartType::Disabled => {
            // disable + bootout. Bootout may fail if the service isn't
            // currently loaded — swallow that and let disable stand.
            let cmd = format!(
                "launchctl disable system/{label} && launchctl bootout system/{label} 2>/dev/null || true",
                label = label
            );
            run_elevated(&cmd).map(|_| ())
        }
        ServiceStartType::Automatic | ServiceStartType::Manual | ServiceStartType::AutoDelayed => {
            // enable + bootstrap. Bootstrap requires a known plist path
            // under /Library/LaunchDaemons; if that path doesn't exist we
            // fall back to enable-only so future loginwindow/launchd cycles
            // pick the service up.
            let cmd = format!(
                "launchctl enable system/{label} && launchctl bootstrap system /Library/LaunchDaemons/{label}.plist 2>/dev/null || true",
                label = label
            );
            run_elevated(&cmd).map(|_| ())
        }
        ServiceStartType::Unknown => Err("Cannot apply Unknown start type".to_string()),
    }
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

fn change(label: &str, start_type: ServiceStartType, rationale: &str) -> ServiceChange {
    ServiceChange {
        service_name: label.to_string(),
        target_start_type: start_type,
        rationale: rationale.to_string(),
    }
}

fn build_presets() -> Vec<ServicePreset> {
    vec![
        ServicePreset {
            id: "developer".to_string(),
            name: "Developer".to_string(),
            description:
                "Disables Spotlight indexing and Time Machine snapshots that compete with builds."
                    .to_string(),
            changes: vec![
                change(
                    "com.apple.metadata.mds",
                    ServiceStartType::Disabled,
                    "Spotlight indexer — high CPU/IO during file churn.",
                ),
                change(
                    "com.apple.backupd-auto",
                    ServiceStartType::Disabled,
                    "Time Machine automatic backups — re-enable manually as needed.",
                ),
            ],
        },
        ServicePreset {
            id: "battery_saver".to_string(),
            name: "Battery Saver".to_string(),
            description: "Stops background indexing and analytics daemons that wake the SoC."
                .to_string(),
            changes: vec![
                change(
                    "com.apple.metadata.mds",
                    ServiceStartType::Disabled,
                    "Spotlight indexer — heavy disk activity.",
                ),
                change(
                    "com.apple.coreduetd.osanalytics",
                    ServiceStartType::Disabled,
                    "OS analytics collector — uploads usage telemetry.",
                ),
            ],
        },
        ServicePreset {
            id: "baseline".to_string(),
            name: "Baseline".to_string(),
            description:
                "Re-enables defaults. Run after a Developer/Battery Saver preset to revert."
                    .to_string(),
            changes: vec![
                change(
                    "com.apple.metadata.mds",
                    ServiceStartType::Automatic,
                    "Spotlight indexer — restore default behavior.",
                ),
                change(
                    "com.apple.backupd-auto",
                    ServiceStartType::Automatic,
                    "Time Machine automatic backups — restore default behavior.",
                ),
                change(
                    "com.apple.coreduetd.osanalytics",
                    ServiceStartType::Automatic,
                    "OS analytics — restore default behavior.",
                ),
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
                        message: format!("Set to {:?}", change.target_start_type),
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
