//! macOS driver recommendation stubs. macOS handles GPU, audio, and Wi-Fi
//! drivers via system updates, so the eventual real implementation will be
//! a thin shim that surfaces `softwareupdate -l` output as recommendations.

use crate::commands::macos::util::STUB_ERR;
use crate::models::drivers::*;

#[tauri::command]
pub async fn get_driver_recommendations() -> Result<Vec<DriverRecommendation>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn install_driver(_winget_id: String) -> Result<String, String> {
    Err(STUB_ERR.into())
}
