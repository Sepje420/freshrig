// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
use crate::models::privacy::{AppPermission, PrivacyCategory, PrivacyRisk, PrivacySetting};
use crate::util::silent_cmd;
use winreg::enums::*;
use winreg::types::FromRegValue;
use winreg::RegKey;

const WIN11_MIN_BUILD: u32 = 22000;
const FILETIME_UNIX_OFFSET_SECS: i64 = 11_644_473_600;
const FILETIME_TICKS_PER_SEC: u64 = 10_000_000;
const CONSENT_STORE_ROOT: &str =
    r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore";
const CAPABILITIES: &[&str] = &[
    "webcam",
    "microphone",
    "location",
    "contacts",
    "broadFileSystemAccess",
    "activity",
];

struct SettingSpec {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    category: PrivacyCategory,
    risk: PrivacyRisk,
    min_build: u32,
    evaluator: fn() -> bool,
}

fn read_dword_hkcu(path: &str, name: &str) -> Option<u32> {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(path)
        .ok()?
        .get_value::<u32, _>(name)
        .ok()
}

fn read_dword_hklm(path: &str, name: &str) -> Option<u32> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(path)
        .ok()?
        .get_value::<u32, _>(name)
        .ok()
}

fn write_dword_hkcu(path: &str, name: &str, value: u32) -> Result<(), String> {
    let (key, _) = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(path)
        .map_err(|e| format!("open/create HKCU\\{}: {}", path, e))?;
    key.set_value(name, &value)
        .map_err(|e| format!("set {}: {}", name, e))
}

fn write_dword_hklm(path: &str, name: &str, value: u32) -> Result<(), String> {
    let (key, _) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .create_subkey(path)
        .map_err(|e| format!("open/create HKLM\\{}: {}", path, e))?;
    key.set_value(name, &value)
        .map_err(|e| format!("set {}: {}", name, e))
}

// ───────── Evaluators (true = private state currently active) ─────────

fn eval_disable_telemetry() -> bool {
    read_dword_hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
    )
    .map(|v| v <= 1)
    .unwrap_or(false)
}

fn eval_disable_diagtrack() -> bool {
    read_dword_hklm(r"SYSTEM\CurrentControlSet\Services\DiagTrack", "Start")
        .map(|v| v == 4)
        .unwrap_or(false)
}

fn eval_disable_error_reporting() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
        "Disabled",
    )
    .map(|v| v == 1)
    .unwrap_or(false)
}

fn eval_disable_advertising_id() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_suggested_content() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "SubscribedContent-338389Enabled",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_start_suggestions() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "SubscribedContent-338388Enabled",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_tips_notifications() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "SubscribedContent-338393Enabled",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_activity_history() -> bool {
    read_dword_hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "PublishUserActivities",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_activity_upload() -> bool {
    read_dword_hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "UploadUserActivities",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_app_launch_tracking() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "Start_TrackProgs",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_clipboard_history() -> bool {
    read_dword_hklm(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "AllowClipboardHistory",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_recall() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "AllowRecallEnablement",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn eval_disable_copilot() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
        "TurnOffWindowsCopilot",
    )
    .map(|v| v == 1)
    .unwrap_or(false)
}

fn eval_disable_web_search() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
        "DisableSearchBoxSuggestions",
    )
    .map(|v| v == 1)
    .unwrap_or(false)
}

fn eval_disable_bing_search() -> bool {
    read_dword_hkcu(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
        "BingSearchEnabled",
    )
    .map(|v| v == 0)
    .unwrap_or(false)
}

fn all_specs() -> Vec<SettingSpec> {
    vec![
        SettingSpec {
            id: "disable_telemetry",
            name: "Disable Windows Telemetry",
            description:
                "Set data collection to minimal. Home/Pro silently clamps 0 → 1 at the OS level.",
            category: PrivacyCategory::Telemetry,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_telemetry,
        },
        SettingSpec {
            id: "disable_diagtrack",
            name: "Disable Diagnostic Tracking Service",
            description: "Stop the Connected User Experiences and Telemetry service (DiagTrack).",
            category: PrivacyCategory::Telemetry,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_diagtrack,
        },
        SettingSpec {
            id: "disable_error_reporting",
            name: "Disable Error Reporting",
            description: "Prevent Windows from sending crash data to Microsoft.",
            category: PrivacyCategory::Telemetry,
            risk: PrivacyRisk::Limited,
            min_build: 0,
            evaluator: eval_disable_error_reporting,
        },
        SettingSpec {
            id: "disable_advertising_id",
            name: "Disable Advertising ID",
            description: "Remove the personalized advertising ID that apps can read.",
            category: PrivacyCategory::Advertising,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_advertising_id,
        },
        SettingSpec {
            id: "disable_suggested_content",
            name: "Disable Settings Suggestions",
            description: "Stop the Settings app from surfacing suggested content.",
            category: PrivacyCategory::Advertising,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_suggested_content,
        },
        SettingSpec {
            id: "disable_start_suggestions",
            name: "Disable Start Menu Suggestions",
            description: "Hide 'Show suggestions occasionally in Start'.",
            category: PrivacyCategory::Suggestions,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_start_suggestions,
        },
        SettingSpec {
            id: "disable_tips_notifications",
            name: "Disable Tips & Tricks Notifications",
            description: "Stop tips, tricks, and welcome notifications.",
            category: PrivacyCategory::Suggestions,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_tips_notifications,
        },
        SettingSpec {
            id: "disable_activity_history",
            name: "Disable Activity History",
            description: "Don't record Timeline / activity history locally.",
            category: PrivacyCategory::Activity,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_activity_history,
        },
        SettingSpec {
            id: "disable_activity_upload",
            name: "Disable Activity Upload",
            description: "Don't send activity history to Microsoft's cloud.",
            category: PrivacyCategory::Activity,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_activity_upload,
        },
        SettingSpec {
            id: "disable_app_launch_tracking",
            name: "Disable App Launch Tracking",
            description: "Don't track most-used apps for Start suggestions.",
            category: PrivacyCategory::Activity,
            risk: PrivacyRisk::Limited,
            min_build: 0,
            evaluator: eval_disable_app_launch_tracking,
        },
        SettingSpec {
            id: "disable_clipboard_history",
            name: "Disable Clipboard History",
            description: "Prevent Windows from remembering clipboard contents.",
            category: PrivacyCategory::Activity,
            risk: PrivacyRisk::Limited,
            min_build: 0,
            evaluator: eval_disable_clipboard_history,
        },
        SettingSpec {
            id: "disable_recall",
            name: "Disable Windows Recall",
            description: "Opt out of the Recall AI screenshot history feature.",
            category: PrivacyCategory::AiCopilot,
            risk: PrivacyRisk::Recommended,
            min_build: WIN11_MIN_BUILD,
            evaluator: eval_disable_recall,
        },
        SettingSpec {
            id: "disable_copilot",
            name: "Disable Windows Copilot",
            description: "Turn off the Copilot sidebar and taskbar button.",
            category: PrivacyCategory::AiCopilot,
            risk: PrivacyRisk::Limited,
            min_build: WIN11_MIN_BUILD,
            evaluator: eval_disable_copilot,
        },
        SettingSpec {
            id: "disable_web_search",
            name: "Disable Search Box Web Suggestions",
            description: "Stop the Start search box from suggesting web results.",
            category: PrivacyCategory::Search,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_web_search,
        },
        SettingSpec {
            id: "disable_bing_search",
            name: "Disable Bing Search Integration",
            description: "Stop Bing from running alongside local file searches.",
            category: PrivacyCategory::Search,
            risk: PrivacyRisk::Recommended,
            min_build: 0,
            evaluator: eval_disable_bing_search,
        },
    ]
}

#[tauri::command]
pub async fn get_privacy_settings() -> Result<Vec<PrivacySetting>, String> {
    tokio::task::spawn_blocking(|| {
        let build = crate::commands::hardware::get_windows_build();
        let mut out = Vec::new();
        for spec in all_specs() {
            if spec.min_build > 0 && build > 0 && build < spec.min_build {
                continue;
            }
            out.push(PrivacySetting {
                id: spec.id.to_string(),
                name: spec.name.to_string(),
                description: spec.description.to_string(),
                category: spec.category,
                risk: spec.risk,
                current_value: (spec.evaluator)(),
                recommended: true,
            });
        }
        out
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))
}

// ───────── Apply ─────────

fn set_diagtrack(disable: bool) -> Result<(), String> {
    let startup = if disable { "Disabled" } else { "Automatic" };
    let script = format!(
        "Set-Service -Name DiagTrack -StartupType {} -ErrorAction Stop",
        startup
    );
    let out = silent_cmd("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| format!("run powershell: {}", e))?;
    if out.status.success() {
        let stop_start = if disable {
            "Stop-Service -Name DiagTrack -Force -ErrorAction SilentlyContinue"
        } else {
            "Start-Service -Name DiagTrack -ErrorAction SilentlyContinue"
        };
        let _ = silent_cmd("powershell")
            .args(["-NoProfile", "-Command", stop_start])
            .output();
        Ok(())
    } else {
        Err(format!(
            "Set-Service failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

fn apply_setting_sync(id: &str, priv_on: bool) -> Result<(), String> {
    match id {
        "disable_telemetry" => write_dword_hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowTelemetry",
            if priv_on { 0 } else { 3 },
        ),
        "disable_diagtrack" => set_diagtrack(priv_on),
        "disable_error_reporting" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
            "Disabled",
            if priv_on { 1 } else { 0 },
        ),
        "disable_advertising_id" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
            "Enabled",
            if priv_on { 0 } else { 1 },
        ),
        "disable_suggested_content" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338389Enabled",
            if priv_on { 0 } else { 1 },
        ),
        "disable_start_suggestions" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338388Enabled",
            if priv_on { 0 } else { 1 },
        ),
        "disable_tips_notifications" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338393Enabled",
            if priv_on { 0 } else { 1 },
        ),
        "disable_activity_history" => write_dword_hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "PublishUserActivities",
            if priv_on { 0 } else { 1 },
        ),
        "disable_activity_upload" => write_dword_hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "UploadUserActivities",
            if priv_on { 0 } else { 1 },
        ),
        "disable_app_launch_tracking" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_TrackProgs",
            if priv_on { 0 } else { 1 },
        ),
        "disable_clipboard_history" => write_dword_hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "AllowClipboardHistory",
            if priv_on { 0 } else { 1 },
        ),
        "disable_recall" => write_dword_hkcu(
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "AllowRecallEnablement",
            if priv_on { 0 } else { 1 },
        ),
        "disable_copilot" => write_dword_hkcu(
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
            "TurnOffWindowsCopilot",
            if priv_on { 1 } else { 0 },
        ),
        "disable_web_search" => write_dword_hkcu(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "DisableSearchBoxSuggestions",
            if priv_on { 1 } else { 0 },
        ),
        "disable_bing_search" => write_dword_hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
            "BingSearchEnabled",
            if priv_on { 0 } else { 1 },
        ),
        other => Err(format!("Unknown privacy setting id: {}", other)),
    }
}

#[tauri::command]
pub async fn apply_privacy_setting(setting_id: String, enable_privacy: bool) -> Result<(), String> {
    let _ = crate::commands::debloat::create_restore_point().await;
    tokio::task::spawn_blocking(move || apply_setting_sync(&setting_id, enable_privacy))
        .await
        .map_err(|e| format!("Task failed: {}", e))?
}

// ───────── App permissions (ConsentStore enumeration) ─────────

fn read_filetime(key: &RegKey, name: &str) -> u64 {
    // REG_QWORD preferred; some builds store REG_BINARY (8 little-endian bytes).
    if let Ok(v) = u64::from_reg_value(&match key.get_raw_value(name) {
        Ok(v) => v,
        Err(_) => return 0,
    }) {
        return v;
    }
    if let Ok(raw) = key.get_raw_value(name) {
        if raw.bytes.len() >= 8 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw.bytes[..8]);
            return u64::from_le_bytes(b);
        }
    }
    0
}

fn filetime_to_iso(ft: u64) -> Option<String> {
    if ft == 0 {
        return None;
    }
    let unix_secs = (ft / FILETIME_TICKS_PER_SEC) as i64 - FILETIME_UNIX_OFFSET_SECS;
    if unix_secs <= 0 {
        return None;
    }
    let (y, mo, d, h, mi, s) = civil_from_unix(unix_secs);
    Some(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, mo, d, h, mi, s
    ))
}

/// Convert Unix seconds (UTC) to (year, month, day, hour, minute, second).
fn civil_from_unix(secs: i64) -> (i32, u32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86400);
    let secs_of_day = secs.rem_euclid(86400) as u32;
    let h = secs_of_day / 3600;
    let mi = (secs_of_day % 3600) / 60;
    let s = secs_of_day % 60;
    // Howard Hinnant's civil_from_days algorithm.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y_adj = if m <= 2 { y + 1 } else { y };
    (y_adj as i32, m, d, h, mi, s)
}

fn format_app_id(subkey: &str, nonpackaged: bool) -> (String, Option<String>) {
    if nonpackaged {
        let path = subkey.replace('#', "\\");
        let name = std::path::Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        (name, Some(path))
    } else {
        let base = subkey.split('_').next().unwrap_or(subkey);
        let name = base.rsplit('.').next().unwrap_or(base).to_string();
        (name, None)
    }
}

fn collect_capability(root: &RegKey, cap: &str, nonpackaged: bool, out: &mut Vec<AppPermission>) {
    for subkey in root.enum_keys().filter_map(|r| r.ok()) {
        if subkey == "NonPackaged" {
            continue;
        }
        let Ok(entry) = root.open_subkey(&subkey) else {
            continue;
        };
        let val: String = entry.get_value("Value").unwrap_or_default();
        let allowed = val == "Allow";
        let start = read_filetime(&entry, "LastUsedTimeStart");
        let stop = read_filetime(&entry, "LastUsedTimeStop");
        let last_used = filetime_to_iso(start);
        let is_active_now = start > 0 && (stop == 0 || stop < start);
        let (app_name, app_path) = format_app_id(&subkey, nonpackaged);
        out.push(AppPermission {
            app_name,
            app_path,
            capability: cap.to_string(),
            allowed,
            last_used,
            is_active_now,
        });
    }
}

#[tauri::command]
pub async fn get_app_permissions() -> Result<Vec<AppPermission>, String> {
    tokio::task::spawn_blocking(|| {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let mut out = Vec::new();
        for cap in CAPABILITIES {
            let root_path = format!("{}\\{}", CONSENT_STORE_ROOT, cap);
            let Ok(root) = hkcu.open_subkey(&root_path) else {
                continue;
            };
            collect_capability(&root, cap, false, &mut out);
            if let Ok(non) = root.open_subkey("NonPackaged") {
                collect_capability(&non, cap, true, &mut out);
            }
        }
        out.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        out
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))
}

#[tauri::command]
pub async fn revoke_app_permission(app_key: String, capability: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || -> Result<(), String> {
        if !CAPABILITIES.contains(&capability.as_str()) {
            return Err(format!("Unknown capability: {}", capability));
        }
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let root_path = format!("{}\\{}", CONSENT_STORE_ROOT, capability);
        let root = hkcu
            .open_subkey_with_flags(&root_path, KEY_READ | KEY_WRITE)
            .map_err(|e| format!("open {}: {}", root_path, e))?;
        let entry = match root.open_subkey_with_flags(&app_key, KEY_READ | KEY_WRITE) {
            Ok(k) => k,
            Err(_) => root
                .open_subkey_with_flags(format!("NonPackaged\\{}", app_key), KEY_READ | KEY_WRITE)
                .map_err(|e| format!("open app entry {}: {}", app_key, e))?,
        };
        entry
            .set_value("Value", &"Deny".to_string())
            .map_err(|e| format!("set Value=Deny: {}", e))
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}
