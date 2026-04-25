//! macOS app install command stubs. The eventual implementation will dispatch
//! to Homebrew (`brew install`, `brew install --cask`) and the Mac App Store
//! CLI (`mas install`).

use crate::commands::macos::util::STUB_ERR;
use crate::models::apps::*;

#[tauri::command]
pub async fn get_app_catalog() -> Result<Vec<AppEntry>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn get_free_disk_space_gb() -> Result<f64, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn check_network_connectivity() -> Result<bool, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn check_winget_available() -> Result<bool, String> {
    // On macOS the equivalent probe will check for `brew` on PATH. Stub for now.
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn install_apps(
    _app_handle: tauri::AppHandle,
    _app_ids: Vec<String>,
) -> Result<(), String> {
    Err(STUB_ERR.into())
}
