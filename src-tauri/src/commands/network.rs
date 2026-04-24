// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
use crate::models::network::{NetworkInterface, WifiProfile};
use crate::util::silent_cmd;
use std::collections::HashMap;
use wmi::WMIConnection;

fn run_checked(program: &str, args: &[&str]) -> Result<String, String> {
    let out = silent_cmd(program)
        .args(args)
        .output()
        .map_err(|e| format!("run {}: {}", program, e))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(format!(
            "{} failed: {}",
            program,
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

#[tauri::command]
pub async fn network_reset_dns() -> Result<(), String> {
    tokio::task::spawn_blocking(|| run_checked("ipconfig", &["/flushdns"]).map(|_| ()))
        .await
        .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn network_reset_full() -> Result<(), String> {
    tokio::task::spawn_blocking(|| {
        run_checked("ipconfig", &["/flushdns"])?;
        run_checked("netsh", &["winsock", "reset"])?;
        run_checked("netsh", &["int", "ip", "reset"])?;
        run_checked("netsh", &["int", "ipv6", "reset"])?;
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn set_dns_servers(
    interface_name: String,
    primary: String,
    secondary: Option<String>,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let addresses = match secondary.as_deref() {
            Some(s) if !s.trim().is_empty() => format!("'{}','{}'", primary, s),
            _ => format!("'{}'", primary),
        };
        let script = format!(
            "Set-DnsClientServerAddress -InterfaceAlias '{}' -ServerAddresses ({}) -ErrorAction Stop",
            interface_name.replace('\'', "''"),
            addresses,
        );
        let out = silent_cmd("powershell")
            .args(["-NoProfile", "-Command", &script])
            .output()
            .map_err(|e| format!("run powershell: {}", e))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Set-DnsClientServerAddress failed: {}",
                String::from_utf8_lossy(&out.stderr)
            ))
        }
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn get_network_interfaces() -> Result<Vec<NetworkInterface>, String> {
    tokio::task::spawn_blocking(|| {
        let wmi = WMIConnection::new().map_err(|e| format!("WMI connect: {}", e))?;
        let results: Vec<HashMap<String, wmi::Variant>> = wmi
            .raw_query(
                "SELECT Name, InterfaceIndex, NetConnectionStatus FROM Win32_NetworkAdapter \
                 WHERE PhysicalAdapter=TRUE AND NetConnectionStatus=2",
            )
            .map_err(|e| format!("WMI query: {}", e))?;

        let mut out = Vec::new();
        for row in results {
            let name = match row.get("Name") {
                Some(wmi::Variant::String(s)) => s.clone(),
                _ => continue,
            };
            let index = match row.get("InterfaceIndex") {
                Some(wmi::Variant::UI4(n)) => *n,
                Some(wmi::Variant::I4(n)) => *n as u32,
                Some(wmi::Variant::UI8(n)) => *n as u32,
                _ => continue,
            };
            out.push(NetworkInterface { name, index });
        }
        Ok(out)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

fn parse_profile_names(output: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in output.lines() {
        if let Some(rest) = line.split_once(':') {
            let label = rest.0.trim();
            if label.eq_ignore_ascii_case("All User Profile")
                || label.eq_ignore_ascii_case("User Profile")
                || label.starts_with("All User Profile")
            {
                let name = rest.1.trim();
                if !name.is_empty() {
                    names.push(name.to_string());
                }
            }
        }
    }
    names
}

fn field_after_colon<'a>(output: &'a str, label: &str) -> Option<&'a str> {
    for line in output.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(label) {
            let rest = rest.trim_start();
            if let Some(after) = rest.strip_prefix(':') {
                return Some(after.trim());
            }
        }
    }
    None
}

#[tauri::command]
pub async fn get_wifi_passwords() -> Result<Vec<WifiProfile>, String> {
    tokio::task::spawn_blocking(|| {
        let listing = run_checked("netsh", &["wlan", "show", "profiles"])?;
        let names = parse_profile_names(&listing);

        let mut out = Vec::new();
        for ssid in names {
            let args = [
                "wlan",
                "show",
                "profile",
                &format!("name={}", ssid),
                "key=clear",
            ];
            let detail = match silent_cmd("netsh").args(args).output() {
                Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
                _ => continue,
            };

            let auth_type = field_after_colon(&detail, "Authentication")
                .unwrap_or("Unknown")
                .to_string();
            let password = field_after_colon(&detail, "Key Content").map(|s| s.to_string());

            out.push(WifiProfile {
                ssid,
                password,
                auth_type,
            });
        }
        Ok(out)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}
