//! macOS privacy settings — surfaces SIP, Gatekeeper, FileVault, the
//! Application Firewall, analytics submission, Siri suggestions, and ad
//! tracking. Most settings are read-only at the CLI layer; SIP and
//! FileVault enrollment require Recovery Mode or System Settings.
//!
//! TCC (Transparency, Consent, Control) database parsing is out of scope:
//! reading `~/Library/Application Support/com.apple.TCC/TCC.db` requires
//! Full Disk Access entitlement that the unsigned dev build doesn't have.
//! `get_app_permissions` therefore returns an empty list.

use crate::commands::macos::util::{run_cmd_lossy, run_elevated};
use crate::models::privacy::*;

const ID_SIP: &str = "macos.sip";
const ID_GATEKEEPER: &str = "macos.gatekeeper";
const ID_FILEVAULT: &str = "macos.filevault";
const ID_FIREWALL: &str = "macos.firewall";
const ID_FIREWALL_STEALTH: &str = "macos.firewall_stealth";
const ID_ANALYTICS: &str = "macos.analytics";
const ID_SIRI_SUGGESTIONS: &str = "macos.siri_suggestions";
const ID_AD_TRACKING: &str = "macos.ad_tracking";

#[tauri::command]
pub async fn get_privacy_settings() -> Result<Vec<PrivacySetting>, String> {
    tokio::task::spawn_blocking(|| {
        let mut out = Vec::new();

        out.push(PrivacySetting {
            id: ID_SIP.to_string(),
            name: "System Integrity Protection".to_string(),
            description: "Kernel-level protection of system files. Toggle in Recovery Mode."
                .to_string(),
            category: PrivacyCategory::Permissions,
            risk: PrivacyRisk::Recommended,
            current_value: read_sip_enabled(),
            recommended: true,
        });

        out.push(PrivacySetting {
            id: ID_GATEKEEPER.to_string(),
            name: "Gatekeeper".to_string(),
            description: "Blocks running unsigned/unnotarized apps.".to_string(),
            category: PrivacyCategory::Permissions,
            risk: PrivacyRisk::Recommended,
            current_value: read_gatekeeper_enabled(),
            recommended: true,
        });

        out.push(PrivacySetting {
            id: ID_FILEVAULT.to_string(),
            name: "FileVault".to_string(),
            description: "Full-disk encryption for the boot volume.".to_string(),
            category: PrivacyCategory::Permissions,
            risk: PrivacyRisk::Recommended,
            current_value: read_filevault_enabled(),
            recommended: true,
        });

        out.push(PrivacySetting {
            id: ID_FIREWALL.to_string(),
            name: "Application Firewall".to_string(),
            description: "Block unsolicited inbound connections to apps.".to_string(),
            category: PrivacyCategory::Permissions,
            risk: PrivacyRisk::Recommended,
            current_value: read_firewall_enabled(),
            recommended: true,
        });

        out.push(PrivacySetting {
            id: ID_FIREWALL_STEALTH.to_string(),
            name: "Firewall stealth mode".to_string(),
            description: "Drop probes silently rather than responding.".to_string(),
            category: PrivacyCategory::Permissions,
            risk: PrivacyRisk::Limited,
            current_value: read_firewall_stealth_enabled(),
            recommended: false,
        });

        out.push(PrivacySetting {
            id: ID_ANALYTICS.to_string(),
            name: "Disable diagnostics & usage submission".to_string(),
            description: "Stops sending crash + usage data to Apple.".to_string(),
            category: PrivacyCategory::Telemetry,
            risk: PrivacyRisk::Recommended,
            current_value: !read_analytics_submit(),
            recommended: true,
        });

        out.push(PrivacySetting {
            id: ID_SIRI_SUGGESTIONS.to_string(),
            name: "Siri Suggestions in apps".to_string(),
            description: "Spotlight + Mail + Messages suggestions backed by Siri.".to_string(),
            category: PrivacyCategory::Suggestions,
            risk: PrivacyRisk::Limited,
            current_value: read_siri_suggestions(),
            recommended: false,
        });

        out.push(PrivacySetting {
            id: ID_AD_TRACKING.to_string(),
            name: "Limit Ad Tracking".to_string(),
            description: "Stop sharing the IDFA with ad networks.".to_string(),
            category: PrivacyCategory::Advertising,
            risk: PrivacyRisk::Recommended,
            current_value: read_ad_tracking_limited(),
            recommended: true,
        });

        Ok(out)
    })
    .await
    .map_err(|e| format!("privacy task failed: {}", e))?
}

#[tauri::command]
pub async fn apply_privacy_setting(setting_id: String, enable_privacy: bool) -> Result<(), String> {
    tokio::task::spawn_blocking(move || match setting_id.as_str() {
        ID_FIREWALL => {
            let state = if enable_privacy { "on" } else { "off" };
            let cmd = format!(
                "/usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate {}",
                state
            );
            run_elevated(&cmd).map(|_| ())
        }
        ID_FIREWALL_STEALTH => {
            let state = if enable_privacy { "on" } else { "off" };
            let cmd = format!(
                "/usr/libexec/ApplicationFirewall/socketfilterfw --setstealthmode {}",
                state
            );
            run_elevated(&cmd).map(|_| ())
        }
        ID_ANALYTICS => {
            let value = if enable_privacy { "false" } else { "true" };
            let cmd = format!(
                "defaults write /Library/Application\\ Support/CrashReporter/DiagnosticMessagesHistory.plist AutoSubmit -bool {}",
                value
            );
            run_elevated(&cmd).map(|_| ())
        }
        ID_AD_TRACKING => {
            let value = if enable_privacy { "true" } else { "false" };
            let out = std::process::Command::new("defaults")
                .args([
                    "write",
                    "com.apple.AdLib",
                    "forceLimitAdTracking",
                    "-bool",
                    value,
                ])
                .output()
                .map_err(|e| format!("Failed to spawn defaults: {}", e))?;
            if !out.status.success() {
                return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
            }
            Ok(())
        }
        ID_SIP | ID_GATEKEEPER | ID_FILEVAULT | ID_SIRI_SUGGESTIONS => Err(
            "This setting must be changed via System Settings or Recovery Mode".to_string(),
        ),
        other => Err(format!("Unknown privacy setting: {}", other)),
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn get_app_permissions() -> Result<Vec<AppPermission>, String> {
    // Reading TCC.db requires Full Disk Access. Without it we'd get an empty
    // result anyway, so return Ok(Vec::new()) and let the panel UI surface
    // its empty state instead of an error.
    Ok(Vec::new())
}

#[tauri::command]
pub async fn revoke_app_permission(_app_key: String, _capability: String) -> Result<(), String> {
    Err("Permission management requires GUI: System Settings → Privacy & Security".to_string())
}

// ---- Probes ----

fn read_sip_enabled() -> bool {
    let raw = run_cmd_lossy("csrutil", &["status"]);
    raw.to_lowercase().contains("enabled")
}

fn read_gatekeeper_enabled() -> bool {
    let raw = run_cmd_lossy("spctl", &["--status"]);
    raw.to_lowercase().contains("assessments enabled")
}

fn read_filevault_enabled() -> bool {
    let raw = run_cmd_lossy("fdesetup", &["status"]);
    raw.to_lowercase().contains("filevault is on")
}

fn read_firewall_enabled() -> bool {
    let raw = run_cmd_lossy(
        "defaults",
        &["read", "/Library/Preferences/com.apple.alf", "globalstate"],
    );
    matches!(raw.trim(), "1" | "2")
}

fn read_firewall_stealth_enabled() -> bool {
    let raw = run_cmd_lossy(
        "defaults",
        &[
            "read",
            "/Library/Preferences/com.apple.alf",
            "stealthenabled",
        ],
    );
    raw.trim() == "1"
}

fn read_analytics_submit() -> bool {
    let raw = run_cmd_lossy(
        "defaults",
        &[
            "read",
            "/Library/Application Support/CrashReporter/DiagnosticMessagesHistory.plist",
            "AutoSubmit",
        ],
    );
    raw.trim() == "1"
}

fn read_siri_suggestions() -> bool {
    let raw = run_cmd_lossy(
        "defaults",
        &[
            "read",
            "com.apple.suggestions",
            "SuggestionsAppLibraryEnabled",
        ],
    );
    // Default when key is absent is "enabled" — treat unreadable as true.
    raw.trim() != "0"
}

fn read_ad_tracking_limited() -> bool {
    let raw = run_cmd_lossy(
        "defaults",
        &["read", "com.apple.AdLib", "forceLimitAdTracking"],
    );
    raw.trim() == "1"
}
