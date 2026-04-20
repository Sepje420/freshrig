use std::collections::HashMap;
use wmi::WMIConnection;

use crate::models::drivers::*;

fn extract_string(map: &HashMap<String, wmi::Variant>, key: &str) -> Option<String> {
    match map.get(key) {
        Some(wmi::Variant::String(s)) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

fn extract_vendor_id(pnp_id: &str) -> Option<String> {
    // PNPDeviceID looks like "PCI\VEN_10DE&DEV_2684&..."
    let upper = pnp_id.to_uppercase();
    if let Some(pos) = upper.find("VEN_") {
        let start = pos + 4;
        let end = upper[start..]
            .find('&')
            .map(|i| start + i)
            .unwrap_or(start + 4);
        if end <= upper.len() {
            return Some(upper[start..end].to_string());
        }
    }
    None
}

fn gpu_download_info(vendor_id: &str) -> (&'static str, &'static str, &'static str) {
    // Returns (vendor_name, download_url, download_page)
    match vendor_id {
        "10DE" => (
            "NVIDIA",
            "https://www.nvidia.com/download/index.aspx",
            "https://www.nvidia.com/download/index.aspx",
        ),
        "1002" => (
            "AMD",
            "https://www.amd.com/en/support",
            "https://www.amd.com/en/support",
        ),
        "8086" => (
            "Intel",
            "https://www.intel.com/content/www/us/en/download-center/home.html",
            "https://www.intel.com/content/www/us/en/download-center/home.html",
        ),
        _ => ("Unknown", "", ""),
    }
}

fn gpu_winget_id(vendor_id: &str) -> Option<&'static str> {
    match vendor_id {
        "10DE" => Some("NVIDIA.GeForceExperience"),
        "8086" => Some("Intel.IntelDriverAndSupportAssistant"),
        _ => None, // AMD Adrenalin not reliably available in winget
    }
}

fn network_winget_id(manufacturer: &str) -> Option<&'static str> {
    let mfr_lower = manufacturer.to_lowercase();
    if mfr_lower.contains("intel") || mfr_lower.contains("killer") || mfr_lower.contains("rivet") {
        Some("Intel.IntelDriverAndSupportAssistant")
    } else {
        None // Realtek, etc. — fallback to support page
    }
}

fn motherboard_support_url(manufacturer: &str, product: &str) -> (String, String) {
    let mfr_lower = manufacturer.to_lowercase();
    let search_product = product.replace(' ', "+");

    if mfr_lower.contains("asus") {
        (
            format!(
                "https://www.asus.com/support/download-center/?q={}",
                search_product
            ),
            "https://www.asus.com/support/download-center/".to_string(),
        )
    } else if mfr_lower.contains("micro-star") || mfr_lower.contains("msi") {
        (
            format!(
                "https://www.msi.com/support/search?keyword={}",
                search_product
            ),
            "https://www.msi.com/support".to_string(),
        )
    } else if mfr_lower.contains("gigabyte") {
        (
            format!(
                "https://www.gigabyte.com/Support/Motherboard?q={}",
                search_product
            ),
            "https://www.gigabyte.com/Support".to_string(),
        )
    } else if mfr_lower.contains("asrock") {
        (
            format!(
                "https://www.asrock.com/support/index.asp?q={}",
                search_product
            ),
            "https://www.asrock.com/support/index.asp".to_string(),
        )
    } else {
        // Generic fallback
        (
            "https://www.intel.com/content/www/us/en/download-center/home.html".to_string(),
            "https://www.intel.com/content/www/us/en/download-center/home.html".to_string(),
        )
    }
}

fn network_driver_info(manufacturer: &str) -> (&'static str, &'static str) {
    let mfr_lower = manufacturer.to_lowercase();
    if mfr_lower.contains("intel") {
        (
            "https://www.intel.com/content/www/us/en/download/18293/intel-network-adapter-driver-for-windows-10.html",
            "https://www.intel.com/content/www/us/en/download-center/home.html",
        )
    } else if mfr_lower.contains("realtek") {
        (
            "https://www.realtek.com/Download/List?cate_id=584",
            "https://www.realtek.com/Download",
        )
    } else if mfr_lower.contains("killer") || mfr_lower.contains("rivet") {
        (
            "https://www.intel.com/content/www/us/en/download/19338/intel-killer-networking-software.html",
            "https://www.intel.com/content/www/us/en/download-center/home.html",
        )
    } else {
        ("", "")
    }
}

fn audio_driver_info(name: &str, manufacturer: &str) -> (&'static str, &'static str, &'static str) {
    let name_lower = name.to_lowercase();
    let mfr_lower = manufacturer.to_lowercase();

    if name_lower.contains("realtek") || mfr_lower.contains("realtek") {
        (
            "Realtek",
            "https://www.realtek.com/Download/List?cate_id=593",
            "https://www.realtek.com/Download",
        )
    } else if name_lower.contains("nvidia") || mfr_lower.contains("nvidia") {
        (
            "NVIDIA",
            "https://www.nvidia.com/download/index.aspx",
            "https://www.nvidia.com/download/index.aspx",
        )
    } else {
        ("Unknown", "", "")
    }
}

#[tauri::command]
pub async fn get_driver_recommendations() -> Result<Vec<DriverRecommendation>, String> {
    tokio::task::spawn_blocking(|| {
        let wmi = WMIConnection::new().map_err(|e| format!("Failed to connect to WMI: {}", e))?;

        let mut recommendations = Vec::new();

        // --- GPU Drivers ---
        let gpu_results: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query("SELECT Name, DriverVersion, DriverDate, PNPDeviceID FROM Win32_VideoController")
            .unwrap_or_default();

        for gpu in &gpu_results {
            let name = extract_string(gpu, "Name").unwrap_or_else(|| "Unknown GPU".to_string());
            let pnp_id = extract_string(gpu, "PNPDeviceID").unwrap_or_default();
            let driver_version = extract_string(gpu, "DriverVersion");
            let driver_date = extract_string(gpu, "DriverDate").map(|d| {
                if d.len() >= 8 {
                    format!("{}-{}-{}", &d[0..4], &d[4..6], &d[6..8])
                } else {
                    d
                }
            });

            let vendor_id = extract_vendor_id(&pnp_id).unwrap_or_default();
            let (vendor_name, download_url, download_page) = gpu_download_info(&vendor_id);
            let winget_id = gpu_winget_id(&vendor_id).map(|s| s.to_string());
            let install_action = if winget_id.is_some() {
                DriverInstallAction::Winget
            } else {
                DriverInstallAction::OpenUrl
            };

            let status = if driver_version.is_none() {
                DriverStatus::Missing
            } else {
                DriverStatus::Unknown
            };

            recommendations.push(DriverRecommendation {
                device_name: name,
                category: DriverCategory::Gpu,
                vendor: vendor_name.to_string(),
                current_version: driver_version,
                current_date: driver_date,
                download_url: download_url.to_string(),
                download_page: download_page.to_string(),
                status,
                winget_id,
                install_action,
            });
        }

        // --- Chipset / Motherboard ---
        let board_results: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query("SELECT Manufacturer, Product FROM Win32_BaseBoard")
            .unwrap_or_default();

        if let Some(board) = board_results.first() {
            let manufacturer = extract_string(board, "Manufacturer").unwrap_or_else(|| "Unknown".to_string());
            let product = extract_string(board, "Product").unwrap_or_else(|| "Unknown".to_string());
            let (download_url, download_page) = motherboard_support_url(&manufacturer, &product);

            recommendations.push(DriverRecommendation {
                device_name: format!("{} {}", manufacturer, product),
                category: DriverCategory::Chipset,
                vendor: manufacturer,
                current_version: None,
                current_date: None,
                download_url,
                download_page,
                status: DriverStatus::Unknown,
                winget_id: None,
                install_action: DriverInstallAction::OpenUrl,
            });
        }

        // --- Network Adapters ---
        let net_results: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query("SELECT Name, Manufacturer FROM Win32_NetworkAdapter WHERE PhysicalAdapter=TRUE")
            .unwrap_or_default();

        // Also query signed driver info for version numbers
        let signed_drivers: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query("SELECT DeviceName, DriverVersion FROM Win32_PnPSignedDriver WHERE DeviceClass='NET'")
            .unwrap_or_default();

        let driver_versions: HashMap<String, String> = signed_drivers
            .iter()
            .filter_map(|d| {
                let name = extract_string(d, "DeviceName")?;
                let version = extract_string(d, "DriverVersion")?;
                Some((name, version))
            })
            .collect();

        for net in &net_results {
            let name = extract_string(net, "Name").unwrap_or_else(|| "Unknown Adapter".to_string());
            let manufacturer = extract_string(net, "Manufacturer").unwrap_or_else(|| "Unknown".to_string());
            let (download_url, download_page) = network_driver_info(&manufacturer);

            if download_url.is_empty() {
                continue; // Skip adapters we can't provide recommendations for
            }

            let current_version = driver_versions.get(&name).cloned();
            let status = if current_version.is_none() {
                DriverStatus::Missing
            } else {
                DriverStatus::Unknown
            };

            let winget_id = network_winget_id(&manufacturer).map(|s| s.to_string());
            let install_action = if winget_id.is_some() {
                DriverInstallAction::Winget
            } else {
                DriverInstallAction::OpenUrl
            };

            recommendations.push(DriverRecommendation {
                device_name: name,
                category: DriverCategory::Network,
                vendor: manufacturer,
                current_version,
                current_date: None,
                download_url: download_url.to_string(),
                download_page: download_page.to_string(),
                status,
                winget_id,
                install_action,
            });
        }

        // --- Audio Devices ---
        let audio_results: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query("SELECT Name, Manufacturer FROM Win32_SoundDevice")
            .unwrap_or_default();

        for audio in &audio_results {
            let name = extract_string(audio, "Name").unwrap_or_else(|| "Unknown Audio".to_string());
            let manufacturer = extract_string(audio, "Manufacturer").unwrap_or_else(|| "Unknown".to_string());
            let (vendor_name, download_url, download_page) = audio_driver_info(&name, &manufacturer);

            if download_url.is_empty() {
                continue;
            }

            // Try to find audio driver version from signed drivers
            let audio_signed: Vec<HashMap<String, wmi::Variant>> = wmi
                .raw_query("SELECT DeviceName, DriverVersion FROM Win32_PnPSignedDriver WHERE DeviceClass='MEDIA'")
                .unwrap_or_default();

            let current_version = audio_signed
                .iter()
                .find_map(|d| {
                    let dev_name = extract_string(d, "DeviceName")?;
                    if dev_name.contains(&name) || name.contains(&dev_name) {
                        extract_string(d, "DriverVersion")
                    } else {
                        None
                    }
                });

            let status = if current_version.is_none() {
                DriverStatus::Missing
            } else {
                DriverStatus::Unknown
            };

            recommendations.push(DriverRecommendation {
                device_name: name,
                category: DriverCategory::Audio,
                vendor: vendor_name.to_string(),
                current_version,
                current_date: None,
                download_url: download_url.to_string(),
                download_page: download_page.to_string(),
                status,
                winget_id: None,
                install_action: DriverInstallAction::OpenUrl,
            });
        }

        Ok(recommendations)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn install_driver(winget_id: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let output = std::process::Command::new("cmd")
            .args([
                "/C",
                &format!(
                    "chcp 65001 >nul && winget install --id {} --silent --accept-package-agreements --accept-source-agreements",
                    winget_id
                ),
            ])
            .output()
            .map_err(|e| format!("Failed to run winget: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success()
            || stdout.contains("Successfully installed")
            || stdout.contains("already installed")
        {
            Ok(format!("Driver tool installed: {}", winget_id))
        } else {
            Err(format!("Install failed: {}\n{}", stdout, stderr))
        }
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
