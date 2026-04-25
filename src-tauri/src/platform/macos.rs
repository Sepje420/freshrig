// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
//
// macOS implementation of the platform abstraction. Mirrors the API surface
// of `platform/linux.rs` and `platform/windows.rs` so cross-platform callers
// can hit `platform::current::*` without cfg gates of their own.
//
// This is the platform-info layer only — actual feature commands live under
// `commands/macos/*` and are stubs until v1.2.0.

use super::types::SystemInfo;
use std::process::Command;

pub fn get_system_info() -> SystemInfo {
    let version = run_cmd("sw_vers", &["-productVersion"])
        .unwrap_or_default()
        .trim()
        .to_string();
    let build = run_cmd("sw_vers", &["-buildVersion"])
        .unwrap_or_default()
        .trim()
        .to_string();
    let hostname = run_cmd("scutil", &["--get", "ComputerName"])
        .unwrap_or_default()
        .trim()
        .to_string();
    let arch = std::env::consts::ARCH.to_string();

    // Parse `sysctl -n kern.boottime` output — format is
    // `{ sec = 1713000000, usec = 0 } Mon Apr 14 12:00:00 2026`.
    let uptime = {
        let output = run_cmd("sysctl", &["-n", "kern.boottime"]).unwrap_or_default();
        if let Some(sec_str) = output.split("sec = ").nth(1) {
            if let Some(sec) = sec_str
                .split(',')
                .next()
                .and_then(|s| s.trim().parse::<u64>().ok())
            {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                now.saturating_sub(sec)
            } else {
                0
            }
        } else {
            0
        }
    };

    SystemInfo {
        os_name: "macOS".into(),
        os_version: version,
        os_build: build,
        hostname,
        architecture: arch,
        uptime_seconds: uptime,
        desktop_environment: Some("Aqua".into()),
        distro_id: Some("macos".into()),
        distro_family: Some("darwin".into()),
    }
}

pub fn get_distro_family() -> Option<String> {
    Some("darwin".into())
}

pub fn is_admin() -> bool {
    nix::unistd::geteuid().is_root()
}

fn run_cmd(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run {}: {}", program, e))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
