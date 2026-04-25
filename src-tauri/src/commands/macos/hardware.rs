//! macOS hardware command stubs. Mirrors the public surface of
//! `commands::linux::hardware`. Real implementations (system_profiler,
//! sysctl, ioreg) land in a later release.

use crate::commands::macos::util::STUB_ERR;
use crate::models::hardware::*;

#[tauri::command]
pub async fn get_hardware_summary() -> Result<HardwareSummary, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn get_driver_issues() -> Result<Vec<DriverIssue>, String> {
    // macOS does not expose Device Manager-style driver issues; the eventual
    // implementation will return an empty vec. For now keep it as a stub
    // error so the dashboard surfaces the "coming soon" banner uniformly.
    Err(STUB_ERR.into())
}

#[tauri::command]
pub fn get_windows_build() -> u32 {
    // Sentinel: 0 means "not Windows / unknown build". Matches the Linux
    // sentinel so the frontend's existing zero-check handles macOS too.
    0
}
