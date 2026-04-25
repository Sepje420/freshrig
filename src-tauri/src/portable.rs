use std::path::PathBuf;

/// Check if FreshRig is running in portable mode.
/// Portable mode is detected by:
/// 1. A `.portable` marker file next to the executable
/// 2. The `FRESHRIG_PORTABLE` environment variable set to "1"
pub fn is_portable() -> bool {
    std::env::var("FRESHRIG_PORTABLE").unwrap_or_default() == "1"
        || std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join(".portable").exists()))
            .unwrap_or(false)
}

/// Get the data directory — either portable (next to exe) or standard (%APPDATA%).
/// Falls back to the standard path if exe-path resolution fails so we never panic
/// during startup (panic in this codepath would crash the tray init in `lib.rs`).
pub fn get_data_dir() -> PathBuf {
    if is_portable() {
        if let Some(dir) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("data")))
        {
            std::fs::create_dir_all(&dir).ok();
            return dir;
        }
        // Portable mode requested but we couldn't resolve the exe path —
        // fall through to the standard %APPDATA% path so the app still boots.
    }
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(appdata).join("com.freshrig.app");
    std::fs::create_dir_all(&dir).ok();
    dir
}

#[tauri::command]
pub fn check_portable_mode() -> bool {
    is_portable()
}
