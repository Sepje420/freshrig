use crate::models::startup::*;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

const PROTECTED_NAMES: &[&str] = &["SecurityHealth", "Windows Defender", "explorer"];

const APPROVED_RUN: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
const APPROVED_RUN_ONCE: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\RunOnce";
const APPROVED_FOLDER: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\StartupFolder";

fn is_protected(name: &str) -> bool {
    let lower = name.to_lowercase();
    PROTECTED_NAMES
        .iter()
        .any(|p| lower.contains(&p.to_lowercase()))
}

fn approved_subpath(source: &StartupSource) -> Option<&'static str> {
    match source {
        StartupSource::RegistryRun => Some(APPROVED_RUN),
        StartupSource::RegistryRunOnce => Some(APPROVED_RUN_ONCE),
        StartupSource::StartupFolder => Some(APPROVED_FOLDER),
        StartupSource::TaskScheduler => None,
    }
}

#[tauri::command]
pub async fn get_startup_entries() -> Result<Vec<StartupEntry>, String> {
    tokio::task::spawn_blocking(|| -> Result<Vec<StartupEntry>, String> {
        let mut entries = Vec::new();

        // Scan 4 registry Run locations
        let locations = [
            (
                HKEY_CURRENT_USER,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                StartupSource::RegistryRun,
                StartupScope::CurrentUser,
                "hkcu-run",
            ),
            (
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                StartupSource::RegistryRun,
                StartupScope::AllUsers,
                "hklm-run",
            ),
            (
                HKEY_CURRENT_USER,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
                StartupSource::RegistryRunOnce,
                StartupScope::CurrentUser,
                "hkcu-runonce",
            ),
            (
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
                StartupSource::RegistryRunOnce,
                StartupScope::AllUsers,
                "hklm-runonce",
            ),
        ];

        for (hive, path, source, scope, slug) in &locations {
            let hkey = RegKey::predef(*hive);
            let Ok(key) = hkey.open_subkey_with_flags(path, KEY_READ) else {
                continue;
            };
            for name_result in key.enum_values() {
                let Ok((val_name, _raw)) = name_result else {
                    continue;
                };
                // Use get_value::<String> to handle UTF-16 REG_SZ/REG_EXPAND_SZ correctly.
                let Ok(command) = key.get_value::<String, _>(&val_name) else {
                    continue;
                };

                let enabled = check_startup_enabled(&val_name, source);
                let id = format!("{}::{}", slug, val_name);

                entries.push(StartupEntry {
                    id,
                    name: val_name,
                    command,
                    source: source.clone(),
                    scope: scope.clone(),
                    enabled,
                    publisher: None,
                    impact: StartupImpact::NotMeasured,
                });
            }
        }

        // Scan user startup folder
        if let Ok(appdata) = std::env::var("APPDATA") {
            let startup_dir = std::path::PathBuf::from(&appdata)
                .join(r"Microsoft\Windows\Start Menu\Programs\Startup");
            if startup_dir.exists() {
                if let Ok(read_dir) = std::fs::read_dir(&startup_dir) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        let lower = filename.to_lowercase();
                        if lower.ends_with(".lnk")
                            || lower.ends_with(".exe")
                            || lower.ends_with(".bat")
                        {
                            let enabled =
                                check_startup_enabled(&filename, &StartupSource::StartupFolder);
                            entries.push(StartupEntry {
                                id: format!("folder::{}", filename),
                                name: filename.clone(),
                                command: entry.path().to_string_lossy().to_string(),
                                source: StartupSource::StartupFolder,
                                scope: StartupScope::CurrentUser,
                                enabled,
                                publisher: None,
                                impact: StartupImpact::NotMeasured,
                            });
                        }
                    }
                }
            }
        }

        Ok(entries)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

fn check_startup_enabled(name: &str, source: &StartupSource) -> bool {
    // StartupApproved lives under HKCU for all sources; absence means "enabled".
    let Some(subpath) = approved_subpath(source) else {
        return true;
    };
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(key) = hkcu.open_subkey_with_flags(subpath, KEY_READ) else {
        return true;
    };
    let Ok(val) = key.get_raw_value(name) else {
        return true;
    };
    if val.bytes.is_empty() {
        return true;
    }
    // Byte 0 == 0x02 or 0x06 means enabled, 0x03 means disabled.
    val.bytes[0] == 0x02 || val.bytes[0] == 0x06
}

#[tauri::command]
pub async fn toggle_startup_entry(id: String, name: String, enabled: bool) -> Result<(), String> {
    if is_protected(&name) {
        return Err(format!(
            "{} is a protected startup item and cannot be disabled",
            name
        ));
    }

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let subpath = if id.starts_with("hkcu-run::") || id.starts_with("hklm-run::") {
            APPROVED_RUN
        } else if id.starts_with("hkcu-runonce::") || id.starts_with("hklm-runonce::") {
            APPROVED_RUN_ONCE
        } else if id.starts_with("folder::") {
            APPROVED_FOLDER
        } else {
            return Err(format!("Unknown startup entry id: {}", id));
        };

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu
            .create_subkey_with_flags(subpath, KEY_READ | KEY_WRITE)
            .map_err(|e| format!("Failed to open StartupApproved: {}", e))?;

        let mut bytes = vec![0u8; 12];
        bytes[0] = if enabled { 0x02 } else { 0x03 };
        if !enabled {
            // Windows FILETIME of the toggle moment (100ns intervals since 1601-01-01).
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let filetime = now
                .saturating_add(11_644_473_600)
                .saturating_mul(10_000_000);
            bytes[4..12].copy_from_slice(&filetime.to_le_bytes());
        }
        let val = RegValue {
            vtype: RegType::REG_BINARY,
            bytes: bytes.into(),
        };
        key.set_raw_value(&name, &val)
            .map_err(|e| format!("Failed to write StartupApproved: {}", e))?;
        Ok(())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}
