//! macOS startup manager stubs. Real implementation will read LaunchAgents
//! plists from `~/Library/LaunchAgents` and `/Library/LaunchAgents`, plus
//! Login Items via `osascript -e 'tell application "System Events" to get
//! the name of every login item'` (or the modern `SMAppService` equivalent
//! when targeting macOS 13+).

use crate::commands::macos::util::STUB_ERR;
use crate::models::startup::*;

#[tauri::command]
pub async fn get_startup_entries() -> Result<Vec<StartupEntry>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn toggle_startup_entry(
    _id: String,
    _name: String,
    _enabled: bool,
) -> Result<(), String> {
    Err(STUB_ERR.into())
}
