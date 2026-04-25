//! macOS privacy stubs. The real version will surface TCC (Transparency,
//! Consent, and Control) entitlements per app via `tccutil`, plus toggles
//! for Spotlight suggestions, Siri analytics, and crash report submission.

use crate::commands::macos::util::STUB_ERR;
use crate::models::privacy::*;

#[tauri::command]
pub async fn get_privacy_settings() -> Result<Vec<PrivacySetting>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn apply_privacy_setting(
    _setting_id: String,
    _enable_privacy: bool,
) -> Result<(), String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn get_app_permissions() -> Result<Vec<AppPermission>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn revoke_app_permission(_app_key: String, _capability: String) -> Result<(), String> {
    Err(STUB_ERR.into())
}
