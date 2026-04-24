//! Linux privacy settings — targets Ubuntu/Debian crash reporting, firewall
//! presence, and distro-level auto-update daemons. Also reports Flatpak app
//! permissions as the Linux analog of Windows app permissions.

use std::fs;
use std::path::PathBuf;

use crate::commands::linux::util::{distro_family, run_cmd, run_cmd_lossy, run_elevated, which};
use crate::models::privacy::*;

const ID_APPORT: &str = "linux.disable_apport";
const ID_WHOOPSIE: &str = "linux.disable_whoopsie";
const ID_POPCON: &str = "linux.disable_popularity_contest";
const ID_FIREWALL: &str = "linux.firewall_enabled";
const ID_AUTO_UPDATES: &str = "linux.auto_updates";

#[tauri::command]
pub async fn get_privacy_settings() -> Result<Vec<PrivacySetting>, String> {
    tokio::task::spawn_blocking(|| {
        let mut out = Vec::new();
        let family = distro_family();

        // Apport / whoopsie (Ubuntu-family).
        if family == "debian" {
            if unit_exists("apport.service") {
                let enabled = systemctl_is_enabled("apport.service");
                out.push(PrivacySetting {
                    id: ID_APPORT.to_string(),
                    name: "Disable Apport crash reporting".to_string(),
                    description: "Apport intercepts crashes and bundles diagnostics for upstream."
                        .to_string(),
                    category: PrivacyCategory::Telemetry,
                    risk: PrivacyRisk::Recommended,
                    current_value: !enabled,
                    recommended: true,
                });
            }
            if unit_exists("whoopsie.service") {
                let enabled = systemctl_is_enabled("whoopsie.service");
                out.push(PrivacySetting {
                    id: ID_WHOOPSIE.to_string(),
                    name: "Disable Whoopsie crash uploader".to_string(),
                    description: "Uploads Apport crashes to Canonical.".to_string(),
                    category: PrivacyCategory::Telemetry,
                    risk: PrivacyRisk::Recommended,
                    current_value: !enabled,
                    recommended: true,
                });
            }
            if std::path::Path::new("/etc/popularity-contest.conf").exists() {
                let participates = fs::read_to_string("/etc/popularity-contest.conf")
                    .map(|s| {
                        s.lines()
                            .any(|l| l.trim().starts_with("PARTICIPATE=\"yes\""))
                    })
                    .unwrap_or(false);
                out.push(PrivacySetting {
                    id: ID_POPCON.to_string(),
                    name: "Disable popularity-contest".to_string(),
                    description: "Sends anonymized package-usage surveys to Debian/Ubuntu."
                        .to_string(),
                    category: PrivacyCategory::Telemetry,
                    risk: PrivacyRisk::Recommended,
                    current_value: !participates,
                    recommended: true,
                });
            }
        }

        // Firewall — this one *recommends turning it ON*, so `current_value`
        // reflects "firewall is already on" directly.
        let firewall_on = firewall_enabled();
        out.push(PrivacySetting {
            id: ID_FIREWALL.to_string(),
            name: "Enable firewall".to_string(),
            description: "Enable ufw (Debian/Ubuntu) or firewalld (Fedora/SUSE/Arch).".to_string(),
            category: PrivacyCategory::Permissions,
            risk: PrivacyRisk::Recommended,
            current_value: firewall_on,
            recommended: true,
        });

        // Auto-updates — "Limited" / gray area. Leave OFF recommended only
        // for users who want manual control.
        if unit_exists("unattended-upgrades.service")
            || unit_exists("dnf-automatic.timer")
            || unit_exists("dnf5-automatic.timer")
        {
            let any_enabled = systemctl_is_enabled("unattended-upgrades.service")
                || systemctl_is_enabled("dnf-automatic.timer")
                || systemctl_is_enabled("dnf5-automatic.timer");
            out.push(PrivacySetting {
                id: ID_AUTO_UPDATES.to_string(),
                name: "Disable automatic updates".to_string(),
                description: "Stops the background update daemon. You'll need to update manually."
                    .to_string(),
                category: PrivacyCategory::Suggestions,
                risk: PrivacyRisk::Limited,
                current_value: !any_enabled,
                recommended: false,
            });
        }

        Ok(out)
    })
    .await
    .map_err(|e| format!("privacy task failed: {}", e))?
}

#[tauri::command]
pub async fn apply_privacy_setting(setting_id: String, enable_privacy: bool) -> Result<(), String> {
    tokio::task::spawn_blocking(move || match setting_id.as_str() {
        ID_APPORT => toggle_unit("apport.service", !enable_privacy),
        ID_WHOOPSIE => toggle_unit("whoopsie.service", !enable_privacy),
        ID_POPCON => toggle_popcon(enable_privacy),
        ID_FIREWALL => apply_firewall(enable_privacy),
        ID_AUTO_UPDATES => apply_auto_updates(!enable_privacy),
        other => Err(format!("Unknown privacy setting: {}", other)),
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn get_app_permissions() -> Result<Vec<AppPermission>, String> {
    tokio::task::spawn_blocking(|| {
        if !which("flatpak") {
            return Ok(Vec::new());
        }
        let list = run_cmd_lossy("flatpak", &["list", "--app", "--columns=application"]);
        let mut out = Vec::new();
        for line in list.lines() {
            let app = line.trim();
            if app.is_empty() || app.starts_with("Application ID") {
                continue;
            }
            let info = run_cmd_lossy("flatpak", &["info", "--show-permissions", app]);
            append_flatpak_permissions(app, &info, &mut out);
        }
        Ok(out)
    })
    .await
    .map_err(|e| format!("permissions task failed: {}", e))?
}

#[tauri::command]
pub async fn revoke_app_permission(app_key: String, capability: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        if !which("flatpak") {
            return Err("flatpak is not installed".to_string());
        }
        let arg = match capability.as_str() {
            "filesystem=host" => "--nofilesystem=host".to_string(),
            "filesystem=home" => "--nofilesystem=home".to_string(),
            "share=network" => "--unshare=network".to_string(),
            "devices=all" => "--nodevice=all".to_string(),
            "sockets=session-bus" => "--nosocket=session-bus".to_string(),
            "sockets=system-bus" => "--nosocket=system-bus".to_string(),
            other => return Err(format!("Unsupported flatpak capability: {}", other)),
        };
        run_cmd("flatpak", &["override", "--user", &arg, &app_key]).map(|_| ())
    })
    .await
    .map_err(|e| format!("revoke task failed: {}", e))?
}

// ---- systemctl helpers ----

fn unit_exists(unit: &str) -> bool {
    let out = run_cmd_lossy("systemctl", &["list-unit-files", unit, "--no-legend"]);
    !out.trim().is_empty()
}

fn systemctl_is_enabled(unit: &str) -> bool {
    let out = run_cmd_lossy("systemctl", &["is-enabled", unit]);
    matches!(
        out.trim(),
        "enabled" | "enabled-runtime" | "static" | "alias"
    )
}

fn toggle_unit(unit: &str, enable: bool) -> Result<(), String> {
    let action = if enable { "enable" } else { "disable" };
    run_elevated("systemctl", &[action, "--now", unit]).map(|_| ())
}

fn toggle_popcon(enable_privacy: bool) -> Result<(), String> {
    let path = PathBuf::from("/etc/popularity-contest.conf");
    if !path.exists() {
        return Err("popularity-contest is not installed".to_string());
    }
    let current = fs::read_to_string(&path).unwrap_or_default();
    let desired = if enable_privacy { "no" } else { "yes" };

    let mut found = false;
    let mut rewritten = String::new();
    for line in current.lines() {
        if line.trim_start().starts_with("PARTICIPATE=") {
            found = true;
            rewritten.push_str(&format!("PARTICIPATE=\"{}\"\n", desired));
        } else {
            rewritten.push_str(line);
            rewritten.push('\n');
        }
    }
    if !found {
        rewritten.push_str(&format!("PARTICIPATE=\"{}\"\n", desired));
    }

    // Write needs root. Do it via pkexec + tee.
    write_elevated("/etc/popularity-contest.conf", &rewritten)
}

fn write_elevated(path: &str, content: &str) -> Result<(), String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("pkexec")
        .args(["tee", path])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("pkexec tee: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(content.as_bytes())
            .map_err(|e| format!("write stdin: {}", e))?;
    }
    let status = child.wait().map_err(|e| format!("wait tee: {}", e))?;
    if !status.success() {
        return Err(format!("tee exited with {}", status));
    }
    Ok(())
}

fn firewall_enabled() -> bool {
    if which("ufw") {
        let out = run_cmd_lossy("ufw", &["status"]);
        return out.to_lowercase().contains("status: active");
    }
    if which("firewall-cmd") {
        let out = run_cmd_lossy("firewall-cmd", &["--state"]);
        return out.trim() == "running";
    }
    false
}

fn apply_firewall(enable_privacy: bool) -> Result<(), String> {
    if which("ufw") {
        let action = if enable_privacy { "enable" } else { "disable" };
        return run_elevated("ufw", &[action]).map(|_| ());
    }
    if which("firewall-cmd") {
        let unit = "firewalld.service";
        return toggle_unit(unit, enable_privacy);
    }
    Err("No supported firewall (ufw/firewalld) is installed".to_string())
}

fn apply_auto_updates(enable: bool) -> Result<(), String> {
    let units = [
        "unattended-upgrades.service",
        "dnf-automatic.timer",
        "dnf5-automatic.timer",
    ];
    let mut last_err: Option<String> = None;
    let mut touched = false;
    for unit in units {
        if unit_exists(unit) {
            touched = true;
            if let Err(e) = toggle_unit(unit, enable) {
                last_err = Some(e);
            }
        }
    }
    if !touched {
        return Err("No auto-update unit is installed".to_string());
    }
    match last_err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

// ---- Flatpak permission parsing ----

fn append_flatpak_permissions(app: &str, info: &str, out: &mut Vec<AppPermission>) {
    // `flatpak info --show-permissions <app>` returns INI-like blocks
    //     [Context]
    //     shared=network;ipc;
    //     sockets=x11;pulseaudio;
    //     devices=all;
    //     filesystems=host;home;
    //
    // We surface only the "risky" ones so the user sees something actionable.
    let mut in_context = false;
    for raw in info.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            in_context = line.eq_ignore_ascii_case("[Context]");
            continue;
        }
        if !in_context || line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        for item in value.split(';') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            let capability = match (key, item) {
                ("filesystems", "host") => Some("filesystem=host"),
                ("filesystems", "home") => Some("filesystem=home"),
                ("shared", "network") => Some("share=network"),
                ("devices", "all") => Some("devices=all"),
                ("sockets", "session-bus") => Some("sockets=session-bus"),
                ("sockets", "system-bus") => Some("sockets=system-bus"),
                _ => None,
            };
            if let Some(cap) = capability {
                out.push(AppPermission {
                    app_name: app.to_string(),
                    app_path: Some(app.to_string()),
                    capability: cap.to_string(),
                    allowed: true,
                    last_used: None,
                    is_active_now: false,
                });
            }
        }
    }
}
