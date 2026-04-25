//! macOS services manager stubs. Real implementation will wrap `launchctl`
//! over LaunchAgents and LaunchDaemons in `/System/Library/`,
//! `/Library/`, and `~/Library/`.

use crate::commands::macos::util::STUB_ERR;
use crate::models::services::{ServiceEntry, ServicePreset, ServicePresetResult};

#[tauri::command]
pub async fn get_services() -> Result<Vec<ServiceEntry>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn set_service_start_type(_name: String, _start_type: String) -> Result<(), String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn get_service_presets() -> Result<Vec<ServicePreset>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn apply_service_preset(
    _app_handle: tauri::AppHandle,
    _preset_id: String,
) -> Result<Vec<ServicePresetResult>, String> {
    Err(STUB_ERR.into())
}
