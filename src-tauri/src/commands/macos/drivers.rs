//! macOS driver recommendations. macOS auto-manages drivers via Software
//! Update (System Settings → General → Software Update). The CLI analog is
//! `softwareupdate --list`. We surface a single recommendation when an
//! update mentions a "Driver" or "Firmware" — install routing opens
//! Software Update via the `macappstore://` URL scheme.

use crate::commands::macos::util::run_cmd_lossy;
use crate::models::drivers::*;

#[tauri::command]
pub async fn get_driver_recommendations() -> Result<Vec<DriverRecommendation>, String> {
    tokio::task::spawn_blocking(|| {
        let mut out = Vec::new();
        let raw = run_cmd_lossy("softwareupdate", &["--list"]);

        // softwareupdate --list lists pending updates; if any line mentions
        // Driver or Firmware, surface a single Apple-vendored recommendation.
        let has_driver_update = raw
            .lines()
            .any(|l| l.contains("Driver") || l.contains("Firmware"));

        if has_driver_update {
            out.push(DriverRecommendation {
                device_name: "Apple system updates (drivers/firmware)".to_string(),
                category: DriverCategory::Other,
                vendor: "Apple".to_string(),
                current_version: None,
                current_date: None,
                download_url: "macappstore://showUpdatesPage".to_string(),
                download_page: "macappstore://showUpdatesPage".to_string(),
                status: DriverStatus::UpdateAvailable,
                install_action: DriverInstallAction::DirectDownload(
                    "macappstore://showUpdatesPage".to_string(),
                ),
                install_label: "Open Software Update".to_string(),
            });
        }

        Ok(out)
    })
    .await
    .map_err(|e| format!("drivers task failed: {}", e))?
}

#[tauri::command]
pub async fn install_driver(_winget_id: String) -> Result<String, String> {
    Err("macOS uses Software Update — open System Settings → General → Software Update".to_string())
}
