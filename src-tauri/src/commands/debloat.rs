use std::process::Command;

use tauri::Emitter;
use winreg::enums::*;
use winreg::RegKey;

use crate::data::debloat_tweaks::get_all_tweaks;
use crate::models::debloat::*;

/// Protected packages that must NEVER be removed
const PROTECTED_PACKAGES: &[&str] = &[
    "Microsoft.WindowsStore",
    "Microsoft.DesktopAppInstaller",
    "Microsoft.WindowsTerminal",
];

const PROTECTED_PREFIXES: &[&str] = &["Microsoft.VCLibs.", "Microsoft.UI.Xaml.", "Microsoft.NET."];

fn is_protected(name: &str) -> bool {
    PROTECTED_PACKAGES.contains(&name)
        || PROTECTED_PREFIXES
            .iter()
            .any(|prefix| name.starts_with(prefix))
}

#[tauri::command]
pub async fn get_debloat_tweaks() -> Result<Vec<DebloatTweak>, String> {
    tokio::task::spawn_blocking(|| {
        let definitions = get_all_tweaks();
        let mut tweaks = Vec::new();

        for def in &definitions {
            let is_applied = check_tweak_applied(def);

            tweaks.push(DebloatTweak {
                id: def.id.to_string(),
                name: def.name.to_string(),
                description: def.description.to_string(),
                tier: def.tier.clone(),
                category: def.category.clone(),
                tweak_type: def.tweak_type.clone(),
                is_applied,
                is_reversible: def.is_reversible,
                warning: def.warning.map(|s| s.to_string()),
            });
        }

        tweaks
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))
}

fn check_tweak_applied(def: &crate::data::debloat_tweaks::TweakDefinition) -> bool {
    match def.tweak_type {
        TweakType::RegistrySet => {
            if def.id == "classic_context_menu" {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                return hkcu
                    .open_subkey(
                        r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
                    )
                    .is_ok();
            }
            if def.id == "disable_location_tracking" {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                if let Ok(key) = hkcu.open_subkey(
                    r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                ) {
                    let val: Result<String, _> = key.get_value("Value");
                    return val.map(|v| v == "Deny").unwrap_or(false);
                }
                return false;
            }
            // Check all registry entries
            for entry in &def.registry_entries {
                let hkey = match entry.hive {
                    "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
                    "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
                    _ => return false,
                };
                match hkey.open_subkey(entry.path) {
                    Ok(key) => {
                        let val: Result<u32, _> = key.get_value(entry.name);
                        if val.unwrap_or(u32::MAX) != entry.value {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }
            }
            !def.registry_entries.is_empty()
        }
        TweakType::AppxRemove => {
            if let Some(appx_name) = def.appx_name {
                let output = Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!(
                            "Get-AppxPackage -Name '*{}*' | Select-Object -First 1 -ExpandProperty Name",
                            appx_name
                        ),
                    ])
                    .output();
                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        // If package NOT found, tweak IS applied (removed)
                        stdout.trim().is_empty()
                    }
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        TweakType::ServiceDisable => {
            if let Some(svc) = def.service_name {
                let output = Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!(
                            "(Get-Service -Name '{}' -ErrorAction SilentlyContinue).StartType",
                            svc
                        ),
                    ])
                    .output();
                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        stdout.trim() == "Disabled"
                    }
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        TweakType::ScheduledTask => false,
    }
}

#[tauri::command]
pub async fn create_restore_point() -> Result<String, String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let description = format!("FreshRig Optimization - {}", timestamp);

    let desc_clone = description.clone();
    tokio::task::spawn_blocking(move || {
        // Enable restore point creation (bypass 24h limit)
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Set-ItemProperty -Path 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore' -Name 'SystemRestorePointCreationFrequency' -Value 0 -Type DWord -ErrorAction SilentlyContinue",
            ])
            .output();

        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop",
                    desc_clone
                ),
            ])
            .output()
            .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

        if output.status.success() {
            Ok(desc_clone)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to create restore point: {}", stderr))
        }
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn apply_debloat_tweaks(
    app_handle: tauri::AppHandle,
    tweak_ids: Vec<String>,
    dry_run: bool,
) -> Result<Vec<DebloatResult>, String> {
    let definitions = get_all_tweaks();
    let mut results = Vec::new();

    // Log file setup
    let log_path = Some(crate::portable::get_data_dir().join("debloat-log.txt"));

    if let Some(ref path) = log_path {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .and_then(|mut f| {
                use std::io::Write;
                writeln!(
                    f,
                    "\n=== FreshRig Debloat Session {} ===\nMode: {}\nTweaks: {:?}",
                    chrono_timestamp(),
                    if dry_run { "DRY RUN" } else { "APPLY" },
                    tweak_ids
                )
            });
    }

    for tweak_id in &tweak_ids {
        let def = match definitions.iter().find(|d| d.id == tweak_id) {
            Some(d) => d,
            None => {
                let result = DebloatResult {
                    tweak_id: tweak_id.clone(),
                    success: false,
                    message: "Unknown tweak ID".to_string(),
                };
                results.push(result.clone());
                let _ = app_handle.emit("debloat-progress", &result);
                continue;
            }
        };

        if dry_run {
            let message = match def.tweak_type {
                TweakType::RegistrySet => format!("Would set {} registry values", {
                    let count = def.registry_entries.len();
                    if def.id == "classic_context_menu" || def.id == "disable_location_tracking" {
                        1
                    } else {
                        count
                    }
                }),
                TweakType::AppxRemove => {
                    format!("Would remove app: {}", def.appx_name.unwrap_or("unknown"))
                }
                TweakType::ServiceDisable => {
                    format!(
                        "Would disable service: {}",
                        def.service_name.unwrap_or("unknown")
                    )
                }
                TweakType::ScheduledTask => "Would disable scheduled task".to_string(),
            };
            let result = DebloatResult {
                tweak_id: tweak_id.clone(),
                success: true,
                message,
            };
            results.push(result.clone());
            let _ = app_handle.emit("debloat-progress", &result);
            continue;
        }

        // Actually apply the tweak
        let result = apply_single_tweak(def);

        if let Some(ref path) = log_path {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .and_then(|mut f| {
                    use std::io::Write;
                    writeln!(
                        f,
                        "[{}] {}: {} — {}",
                        chrono_timestamp(),
                        def.id,
                        if result.success { "OK" } else { "FAIL" },
                        result.message
                    )
                });
        }

        let _ = app_handle.emit("debloat-progress", &result);
        results.push(result);
    }

    Ok(results)
}

fn apply_single_tweak(def: &crate::data::debloat_tweaks::TweakDefinition) -> DebloatResult {
    let tweak_id = def.id.to_string();

    match def.tweak_type {
        TweakType::RegistrySet => {
            // Special case: classic context menu
            if def.id == "classic_context_menu" {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                match hkcu.create_subkey(
                    r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
                ) {
                    Ok((key, _)) => {
                        let _ = key.set_value("", &"");
                        return DebloatResult {
                            tweak_id,
                            success: true,
                            message: "Classic context menu enabled (restart Explorer to apply)"
                                .to_string(),
                        };
                    }
                    Err(e) => {
                        return DebloatResult {
                            tweak_id,
                            success: false,
                            message: format!("Failed: {}", e),
                        };
                    }
                }
            }

            // Special case: location tracking (string value)
            if def.id == "disable_location_tracking" {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                match hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location") {
                    Ok((key, _)) => {
                        match key.set_value("Value", &"Deny") {
                            Ok(_) => return DebloatResult { tweak_id, success: true, message: "Location tracking disabled".to_string() },
                            Err(e) => return DebloatResult { tweak_id, success: false, message: format!("Failed: {}", e) },
                        }
                    }
                    Err(e) => return DebloatResult { tweak_id, success: false, message: format!("Failed: {}", e) },
                }
            }

            // Standard registry set
            for entry in &def.registry_entries {
                let hkey = match entry.hive {
                    "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
                    "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
                    _ => {
                        return DebloatResult {
                            tweak_id,
                            success: false,
                            message: "Invalid registry hive".to_string(),
                        };
                    }
                };
                match hkey.create_subkey(entry.path) {
                    Ok((key, _)) => {
                        if let Err(e) = key.set_value(entry.name, &entry.value) {
                            return DebloatResult {
                                tweak_id,
                                success: false,
                                message: format!("Failed to set {}: {}", entry.name, e),
                            };
                        }
                    }
                    Err(e) => {
                        return DebloatResult {
                            tweak_id,
                            success: false,
                            message: format!("Failed to open registry key: {}", e),
                        };
                    }
                }
            }
            DebloatResult {
                tweak_id,
                success: true,
                message: format!("{} applied successfully", def.name),
            }
        }
        TweakType::AppxRemove => {
            let appx_name = match def.appx_name {
                Some(name) => name,
                None if def.id == "remove_onedrive" => {
                    // Special OneDrive removal
                    let output = Command::new("cmd")
                        .args([
                            "/C",
                            "taskkill /f /im OneDrive.exe >nul 2>&1 & %SystemRoot%\\SysWOW64\\OneDriveSetup.exe /uninstall 2>nul || %SystemRoot%\\System32\\OneDriveSetup.exe /uninstall 2>nul",
                        ])
                        .output();
                    return match output {
                        Ok(out) if out.status.success() => DebloatResult {
                            tweak_id,
                            success: true,
                            message: "OneDrive removed".to_string(),
                        },
                        Ok(_) => DebloatResult {
                            tweak_id,
                            success: true,
                            message: "OneDrive removal attempted (may already be removed)"
                                .to_string(),
                        },
                        Err(e) => DebloatResult {
                            tweak_id,
                            success: false,
                            message: format!("Failed: {}", e),
                        },
                    };
                }
                None => {
                    return DebloatResult {
                        tweak_id,
                        success: false,
                        message: "No package name defined".to_string(),
                    };
                }
            };

            // Safety check
            if is_protected(appx_name) {
                return DebloatResult {
                    tweak_id,
                    success: false,
                    message: format!("{} is a protected package and cannot be removed", appx_name),
                };
            }

            let output = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    &format!(
                        "Get-AppxPackage -Name '*{}*' | Remove-AppxPackage -ErrorAction Stop",
                        appx_name
                    ),
                ])
                .output();

            match output {
                Ok(out) if out.status.success() => DebloatResult {
                    tweak_id,
                    success: true,
                    message: format!("{} removed", appx_name),
                },
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    if stderr.contains("not found") || stderr.contains("does not exist") {
                        DebloatResult {
                            tweak_id,
                            success: true,
                            message: format!("{} already removed", appx_name),
                        }
                    } else {
                        DebloatResult {
                            tweak_id,
                            success: false,
                            message: format!("Failed: {}", stderr.lines().next().unwrap_or("")),
                        }
                    }
                }
                Err(e) => DebloatResult {
                    tweak_id,
                    success: false,
                    message: format!("Failed to run PowerShell: {}", e),
                },
            }
        }
        TweakType::ServiceDisable => {
            let svc = match def.service_name {
                Some(s) => s,
                None => {
                    return DebloatResult {
                        tweak_id,
                        success: false,
                        message: "No service name defined".to_string(),
                    };
                }
            };

            let output = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    &format!(
                        "Stop-Service -Name '{}' -Force -ErrorAction SilentlyContinue; Set-Service -Name '{}' -StartupType Disabled -ErrorAction Stop",
                        svc, svc
                    ),
                ])
                .output();

            // Also set registry entries if present (e.g., telemetry)
            for entry in &def.registry_entries {
                let hkey = match entry.hive {
                    "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
                    "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
                    _ => continue,
                };
                if let Ok((key, _)) = hkey.create_subkey(entry.path) {
                    let _ = key.set_value(entry.name, &entry.value);
                }
            }

            match output {
                Ok(out) if out.status.success() => DebloatResult {
                    tweak_id,
                    success: true,
                    message: format!("Service {} disabled", svc),
                },
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    DebloatResult {
                        tweak_id,
                        success: false,
                        message: format!("Failed: {}", stderr.lines().next().unwrap_or("")),
                    }
                }
                Err(e) => DebloatResult {
                    tweak_id,
                    success: false,
                    message: format!("Failed: {}", e),
                },
            }
        }
        TweakType::ScheduledTask => DebloatResult {
            tweak_id,
            success: false,
            message: "Not implemented".to_string(),
        },
    }
}

#[tauri::command]
pub async fn check_admin_elevation() -> Result<bool, String> {
    // Try to open a protected registry key to check admin status
    tokio::task::spawn_blocking(|| {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            KEY_WRITE,
        )
        .is_ok()
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))
}

#[tauri::command]
pub async fn get_installed_appx_packages() -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(|| {
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-AppxPackage | Select-Object -ExpandProperty Name",
            ])
            .output()
            .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

fn chrono_timestamp() -> String {
    let now = std::time::SystemTime::now();
    let secs = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}
