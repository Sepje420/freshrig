//! macOS network tool stubs. The real version will wrap `dscacheutil`,
//! `killall mDNSResponder`, `networksetup -setdnsservers`, and parse Wi-Fi
//! profiles from `security find-generic-password` against the Keychain.

use crate::commands::macos::util::STUB_ERR;
use crate::models::network::{NetworkInterface, WifiProfile};

#[tauri::command]
pub async fn network_reset_dns() -> Result<(), String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn network_reset_full() -> Result<(), String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn set_dns_servers(
    _interface_name: String,
    _primary: String,
    _secondary: Option<String>,
) -> Result<(), String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn get_network_interfaces() -> Result<Vec<NetworkInterface>, String> {
    Err(STUB_ERR.into())
}

#[tauri::command]
pub async fn get_wifi_passwords() -> Result<Vec<WifiProfile>, String> {
    Err(STUB_ERR.into())
}
