//! macOS disk cleanup stubs. The real version will scan
//! `~/Library/Caches`, `~/.Trash`, `/Library/Logs`, Xcode DerivedData,
//! Homebrew cache, and the user-level mail / messages caches.

use tauri::AppHandle;

use crate::commands::macos::util::STUB_ERR;
use crate::models::cleanup::*;

#[tauri::command]
pub async fn scan_cleanup(_app_handle: AppHandle) -> Result<Vec<CleanupCategory>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn run_cleanup(
    _app_handle: AppHandle,
    _category_ids: Vec<String>,
) -> Result<Vec<CleanupResult>, String> {
    Err(STUB_ERR.into())
}
