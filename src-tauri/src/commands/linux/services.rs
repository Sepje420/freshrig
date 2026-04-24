//! Linux services manager — wraps systemd via `systemctl`.
//!
//! Matches the public command surface of the Windows `services` module so the
//! frontend is OS-agnostic.

use serde::Deserialize;
use tauri::Emitter;

use crate::commands::linux::util::{is_root, run_cmd, run_cmd_lossy, run_elevated, which};
use crate::models::services::{
    ServiceChange, ServiceEntry, ServicePreset, ServicePresetResult, ServiceStartType, ServiceState,
};

/// Systemd units that should never be disabled — doing so breaks login,
/// logging, hardware hot-plug, IPC, or auth.
const NEVER_DISABLE: &[&str] = &[
    "systemd-journald",
    "systemd-logind",
    "systemd-udevd",
    "systemd-resolved",
    "systemd-networkd",
    "NetworkManager",
    "dbus",
    "polkit",
    "getty@tty1",
];

fn strip_service(name: &str) -> &str {
    name.strip_suffix(".service").unwrap_or(name)
}

fn is_protected(name: &str) -> bool {
    let short = strip_service(name);
    NEVER_DISABLE
        .iter()
        .any(|p| p.eq_ignore_ascii_case(short) || p.eq_ignore_ascii_case(name))
}

/// systemctl list-units --output=json row.
#[derive(Debug, Deserialize)]
struct SystemdUnit {
    #[serde(default)]
    unit: String,
    #[serde(default)]
    load: String,
    #[serde(default)]
    active: String,
    #[serde(default)]
    sub: String,
    #[serde(default)]
    description: String,
}

fn is_enabled_probe(name: &str) -> String {
    let out = run_cmd_lossy("systemctl", &["is-enabled", name]);
    out.trim().to_string()
}

fn map_enabled_to_start_type(s: &str) -> ServiceStartType {
    match s {
        "enabled" | "enabled-runtime" | "alias" => ServiceStartType::Automatic,
        "disabled" | "masked" | "masked-runtime" => ServiceStartType::Disabled,
        "static" | "indirect" | "linked" | "linked-runtime" | "generated" | "transient" => {
            ServiceStartType::Manual
        }
        _ => ServiceStartType::Unknown,
    }
}

fn map_active_to_state(active: &str, sub: &str) -> ServiceState {
    match active {
        "active" if sub == "running" => ServiceState::Running,
        "active" => ServiceState::Running,
        "activating" => ServiceState::StartPending,
        "deactivating" => ServiceState::StopPending,
        "inactive" | "failed" => ServiceState::Stopped,
        _ => ServiceState::Unknown,
    }
}

#[tauri::command]
pub async fn get_services() -> Result<Vec<ServiceEntry>, String> {
    tokio::task::spawn_blocking(|| {
        let json = run_cmd(
            "systemctl",
            &[
                "list-units",
                "--type=service",
                "--all",
                "--no-pager",
                "--output=json",
            ],
        )
        .map_err(|e| format!("systemctl list-units failed: {}", e))?;

        let units: Vec<SystemdUnit> =
            serde_json::from_str(&json).map_err(|e| format!("parse systemctl json: {}", e))?;

        let mut out = Vec::with_capacity(units.len());
        for unit in units {
            if !unit.unit.ends_with(".service") {
                continue;
            }
            if unit.load == "not-found" {
                continue;
            }
            let short = strip_service(&unit.unit).to_string();
            let enabled = is_enabled_probe(&unit.unit);
            let start_type = map_enabled_to_start_type(&enabled);
            let current_state = map_active_to_state(&unit.active, &unit.sub);
            let is_protected_flag = is_protected(&unit.unit);

            out.push(ServiceEntry {
                name: short,
                display_name: if unit.description.is_empty() {
                    unit.unit.clone()
                } else {
                    unit.description.clone()
                },
                description: unit.description,
                start_type,
                current_state,
                is_protected: is_protected_flag,
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

fn parse_start_type(s: &str) -> Result<ServiceStartType, String> {
    match s.to_ascii_lowercase().as_str() {
        "automatic" | "auto" => Ok(ServiceStartType::Automatic),
        "autodelayed" | "automaticdelayedstart" | "delayed" => Ok(ServiceStartType::AutoDelayed),
        "manual" => Ok(ServiceStartType::Manual),
        "disabled" => Ok(ServiceStartType::Disabled),
        other => Err(format!("Unknown start type: {}", other)),
    }
}

fn unit_name(raw: &str) -> String {
    if raw.ends_with(".service") {
        raw.to_string()
    } else {
        format!("{}.service", raw)
    }
}

fn apply_start_type_sync(name: &str, start_type: &ServiceStartType) -> Result<(), String> {
    if is_protected(name) {
        return Err(format!(
            "'{}' is protected — disabling it can break the system",
            name
        ));
    }
    let unit = unit_name(name);
    match start_type {
        ServiceStartType::Automatic => {
            run_elevated("systemctl", &["enable", "--now", &unit]).map(|_| ())
        }
        ServiceStartType::AutoDelayed => run_elevated("systemctl", &["enable", &unit]).map(|_| ()),
        ServiceStartType::Manual => {
            // "Manual" on systemd ≈ not auto-started on boot but still
            // runnable. Disable auto-start, but keep the service usable.
            run_elevated("systemctl", &["disable", &unit]).map(|_| ())
        }
        ServiceStartType::Disabled => {
            // Stop + disable. We use `disable --now` which covers both, and
            // swallow stop errors for units that are already inactive.
            run_elevated("systemctl", &["disable", "--now", &unit]).map(|_| ())
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

fn change(name: &str, start_type: ServiceStartType, rationale: &str) -> ServiceChange {
    ServiceChange {
        service_name: name.to_string(),
        target_start_type: start_type,
        rationale: rationale.to_string(),
    }
}

fn has_printer_configured() -> bool {
    if !which("lpstat") {
        return false;
    }
    let out = run_cmd_lossy("lpstat", &["-p"]);
    !out.trim().is_empty()
}

fn build_presets() -> Vec<ServicePreset> {
    let mut performance_changes = vec![
        change(
            "bluetooth",
            ServiceStartType::Disabled,
            "Bluetooth stack — disable if you don't use Bluetooth devices.",
        ),
        change(
            "packagekit",
            ServiceStartType::Disabled,
            "PackageKit daemon — wakes up in the background to check for updates.",
        ),
    ];
    if !has_printer_configured() {
        performance_changes.insert(
            0,
            change(
                "cups",
                ServiceStartType::Disabled,
                "CUPS print server — no printer is configured on this system.",
            ),
        );
    }

    vec![
        ServicePreset {
            id: "gaming".to_string(),
            name: "Gaming".to_string(),
            description:
                "Disables background indexing and discovery services that compete with games."
                    .to_string(),
            changes: vec![
                change(
                    "tracker-miner-fs-3",
                    ServiceStartType::Disabled,
                    "GNOME Tracker file indexer — heavy disk/CPU activity.",
                ),
                change(
                    "fwupd-refresh.timer",
                    ServiceStartType::Disabled,
                    "Firmware update probe — run manually instead of in the background.",
                ),
                change(
                    "ModemManager",
                    ServiceStartType::Disabled,
                    "Modem daemon — unused on desktops without cellular hardware.",
                ),
                change(
                    "bluetooth",
                    ServiceStartType::Disabled,
                    "Bluetooth stack — disable unless you game with a wireless controller.",
                ),
            ],
        },
        ServicePreset {
            id: "privacy".to_string(),
            name: "Privacy".to_string(),
            description:
                "Stops daemons that broadcast on the local network or report crashes upstream."
                    .to_string(),
            changes: vec![
                change(
                    "avahi-daemon",
                    ServiceStartType::Disabled,
                    "mDNS/Zeroconf broadcaster — advertises your hostname on the LAN.",
                ),
                change(
                    "cups-browsed",
                    ServiceStartType::Disabled,
                    "CUPS network printer browser — announces and searches for printers.",
                ),
                change(
                    "whoopsie",
                    ServiceStartType::Disabled,
                    "Ubuntu crash uploader — sends crash reports to Canonical.",
                ),
                change(
                    "apport",
                    ServiceStartType::Disabled,
                    "Apport crash interception — pairs with whoopsie for crash telemetry.",
                ),
            ],
        },
        ServicePreset {
            id: "performance".to_string(),
            name: "Performance".to_string(),
            description: "Shuts down low-value background daemons to cut RAM and wake-ups."
                .to_string(),
            changes: performance_changes,
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
        let running_as_root = is_root();
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
                        message: if running_as_root {
                            e
                        } else {
                            format!("{e} (hint: re-run with elevation)")
                        },
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
