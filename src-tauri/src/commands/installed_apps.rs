use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
    pub display_name: String,
    pub display_version: Option<String>,
    pub publisher: Option<String>,
    pub install_date: Option<String>,
    pub install_location: Option<String>,
    pub estimated_size_kb: Option<u32>,
}

#[tauri::command]
pub async fn get_installed_apps() -> Result<Vec<InstalledApp>, String> {
    tokio::task::spawn_blocking(scan_registry_apps)
        .await
        .map_err(|e| format!("Task failed: {}", e))
}

fn scan_registry_apps() -> Vec<InstalledApp> {
    let mut apps = Vec::new();
    let paths = [
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_CURRENT_USER,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
    ];

    for (hive, path) in &paths {
        let hkey = RegKey::predef(*hive);
        if let Ok(key) = hkey.open_subkey_with_flags(path, KEY_READ) {
            for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
                if let Ok(sub) = key.open_subkey_with_flags(&subkey_name, KEY_READ) {
                    let display_name: Result<String, _> = sub.get_value("DisplayName");
                    if let Ok(name) = display_name {
                        if name.trim().is_empty() {
                            continue;
                        }
                        // Skip system components
                        if sub.get_value::<u32, _>("SystemComponent").unwrap_or(0) == 1 {
                            continue;
                        }
                        // Skip Windows updates
                        if name.starts_with("KB") && name.len() <= 10 {
                            continue;
                        }

                        apps.push(InstalledApp {
                            display_name: name,
                            display_version: sub.get_value("DisplayVersion").ok(),
                            publisher: sub.get_value("Publisher").ok(),
                            install_date: sub.get_value("InstallDate").ok(),
                            install_location: sub.get_value("InstallLocation").ok(),
                            estimated_size_kb: sub.get_value("EstimatedSize").ok(),
                        });
                    }
                }
            }
        }
    }

    // Deduplicate by display name
    apps.sort_by(|a, b| {
        a.display_name
            .to_lowercase()
            .cmp(&b.display_name.to_lowercase())
    });
    apps.dedup_by(|a, b| a.display_name.to_lowercase() == b.display_name.to_lowercase());
    apps
}

#[tauri::command]
pub async fn check_apps_installed(
    winget_ids: Vec<String>,
    catalog_names: Vec<String>,
) -> Result<Vec<String>, String> {
    let installed = tokio::task::spawn_blocking(scan_registry_apps)
        .await
        .map_err(|e| format!("Task failed: {}", e))?;

    let installed_names: Vec<String> = installed
        .iter()
        .map(|a| a.display_name.to_lowercase())
        .collect();

    let mut found_ids = Vec::new();

    for (id, name) in winget_ids.iter().zip(catalog_names.iter()) {
        let name_lower = name.to_lowercase();
        let is_installed = installed_names.iter().any(|installed_name| {
            installed_name == &name_lower
                || installed_name.contains(&name_lower)
                || name_lower.contains(installed_name)
                || id
                    .split('.')
                    .next_back()
                    .map(|part| installed_name.contains(&part.to_lowercase()))
                    .unwrap_or(false)
        });

        if is_installed {
            found_ids.push(id.clone());
        }
    }

    Ok(found_ids)
}
