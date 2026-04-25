//! macOS health report. Structs are duplicated verbatim from the Windows
//! report (same camelCase derives) so the frontend consumes identical JSON
//! on every OS.

#![allow(dead_code)]

use serde::Serialize;

use crate::commands::macos::util::run_cmd_lossy;

// ---- Shared struct surface (kept byte-for-byte the same as Windows) ----

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportData {
    pub generated_at: String,
    pub app_version: String,
    pub overall_grade: String,
    pub overall_score: u32,
    pub system: SystemReport,
    pub hardware: HardwareReport,
    pub drives: Vec<DriveSmartReport>,
    pub battery: Option<BatteryReport>,
    pub security: SecurityReport,
    pub drivers: DriverSummaryReport,
    pub software_count: u32,
    pub startup_count: u32,
    pub startup_enabled_count: u32,
    pub reliability_index: Option<f32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemReport {
    pub hostname: String,
    pub os_name: String,
    pub os_build: String,
    pub uptime_hours: u64,
    pub windows_activated: bool,
    pub windows_edition: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareReport {
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub ram_total_gb: f32,
    pub ram_slots: Vec<RamSlotReport>,
    pub gpus: Vec<String>,
    pub motherboard: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RamSlotReport {
    pub capacity_gb: f32,
    pub speed_mhz: u32,
    pub manufacturer: String,
    pub part_number: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveSmartReport {
    pub model: String,
    pub size_gb: u64,
    pub health_status: String,
    pub temperature_c: Option<u32>,
    pub power_on_hours: Option<u64>,
    pub wear_percentage: Option<u32>,
    pub read_errors_total: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryReport {
    pub design_capacity_mwh: u32,
    pub full_charge_capacity_mwh: u32,
    pub cycle_count: u32,
    pub health_percent: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityReport {
    pub antivirus_name: Option<String>,
    pub antivirus_enabled: bool,
    pub antivirus_up_to_date: bool,
    pub firewall_enabled: bool,
    pub bitlocker_status: String,
    pub tpm_present: bool,
    pub tpm_enabled: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverSummaryReport {
    pub total: u32,
    pub with_errors: u32,
    pub error_devices: Vec<String>,
}

// ---- Main command ----

#[tauri::command]
pub async fn generate_health_report(app_version: String) -> Result<ReportData, String> {
    let hardware_summary = crate::commands::macos::hardware::get_hardware_summary()
        .await
        .ok();
    let startup_entries = crate::commands::macos::startup::get_startup_entries()
        .await
        .unwrap_or_default();

    tokio::task::spawn_blocking(move || {
        let system = read_system(&hardware_summary);
        let hardware = read_hardware(&hardware_summary);
        let drives = read_drives();
        let battery = read_battery();
        let security = read_security();
        let drivers = DriverSummaryReport {
            total: hardware_summary
                .as_ref()
                .map(|h| h.network_adapters.len() as u32 + h.audio_devices.len() as u32)
                .unwrap_or(0),
            with_errors: 0,
            error_devices: Vec::new(),
        };

        let software_count = read_software_count();
        let startup_count = startup_entries.len() as u32;
        let startup_enabled_count = startup_entries.iter().filter(|e| e.enabled).count() as u32;

        let generated_at = chrono_like_now();

        let mut report = ReportData {
            generated_at,
            app_version,
            overall_grade: String::new(),
            overall_score: 0,
            system,
            hardware,
            drives,
            battery,
            security,
            drivers,
            software_count,
            startup_count,
            startup_enabled_count,
            reliability_index: None,
        };

        let (grade, score) = compute_grade(&report);
        report.overall_grade = grade;
        report.overall_score = score;

        Ok::<ReportData, String>(report)
    })
    .await
    .map_err(|e| format!("report task failed: {}", e))?
}

// ---- Section builders ----

fn read_system(hw: &Option<crate::models::hardware::HardwareSummary>) -> SystemReport {
    let hostname = hw
        .as_ref()
        .map(|h| h.system.hostname.clone())
        .unwrap_or_default();
    let os_build = hw
        .as_ref()
        .map(|h| h.system.os_build.clone())
        .unwrap_or_default();
    let uptime_hours = hw
        .as_ref()
        .map(|h| h.system.uptime_seconds / 3600)
        .unwrap_or(0);

    SystemReport {
        hostname,
        os_name: "macOS".to_string(),
        os_build,
        uptime_hours,
        windows_activated: false,
        windows_edition: String::new(),
    }
}

fn read_hardware(hw: &Option<crate::models::hardware::HardwareSummary>) -> HardwareReport {
    let Some(hw) = hw else {
        return HardwareReport {
            cpu_name: String::new(),
            cpu_cores: 0,
            cpu_threads: 0,
            ram_total_gb: 0.0,
            ram_slots: vec![],
            gpus: vec![],
            motherboard: String::new(),
        };
    };

    let ram_slots = read_ram_slots().unwrap_or_else(|| {
        vec![RamSlotReport {
            capacity_gb: hw.system.total_ram_gb as f32,
            speed_mhz: 0,
            manufacturer: String::new(),
            part_number: String::new(),
        }]
    });

    let gpus = hw
        .gpus
        .iter()
        .map(|g| {
            if g.manufacturer.is_empty() {
                g.name.clone()
            } else {
                format!("{} {}", g.manufacturer, g.name)
            }
        })
        .collect();

    let motherboard = format!("{} {}", hw.motherboard.manufacturer, hw.motherboard.product)
        .trim()
        .to_string();

    HardwareReport {
        cpu_name: hw.cpu.name.clone(),
        cpu_cores: hw.cpu.cores,
        cpu_threads: hw.cpu.threads,
        ram_total_gb: hw.system.total_ram_gb as f32,
        ram_slots,
        gpus,
        motherboard,
    }
}

fn read_ram_slots() -> Option<Vec<RamSlotReport>> {
    let json = run_cmd_lossy("system_profiler", &["SPMemoryDataType", "-json"]);
    let parsed: serde_json::Value = serde_json::from_str(&json).ok()?;
    let arr = parsed.get("SPMemoryDataType").and_then(|v| v.as_array())?;

    let mut slots = Vec::new();
    for top in arr {
        // Apple Silicon: top-level item describes the whole SoC RAM with
        // `dimm_size`, `dimm_speed`, `dimm_manufacturer`, `dimm_part_number`.
        if let Some(slot) = build_slot(top) {
            slots.push(slot);
            continue;
        }
        // Intel: nested `_items` with one row per DIMM.
        if let Some(items) = top.get("_items").and_then(|v| v.as_array()) {
            for item in items {
                if let Some(slot) = build_slot(item) {
                    slots.push(slot);
                }
            }
        }
    }

    if slots.is_empty() {
        None
    } else {
        Some(slots)
    }
}

fn build_slot(item: &serde_json::Value) -> Option<RamSlotReport> {
    let cap = item
        .get("dimm_size")
        .or_else(|| item.get("SPMemoryDataType"))
        .and_then(|v| v.as_str())
        .map(parse_memory_size_gb);
    let cap = match cap {
        Some(c) if c > 0.0 => c,
        _ => return None,
    };

    let speed = item
        .get("dimm_speed")
        .and_then(|v| v.as_str())
        .map(parse_memory_speed_mhz)
        .unwrap_or(0);
    let manu = item
        .get("dimm_manufacturer")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let part = item
        .get("dimm_part_number")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Some(RamSlotReport {
        capacity_gb: cap,
        speed_mhz: speed,
        manufacturer: manu,
        part_number: part,
    })
}

fn parse_memory_size_gb(s: &str) -> f32 {
    let mut parts = s.split_whitespace();
    let num = parts
        .next()
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0);
    let unit = parts.next().unwrap_or("").to_uppercase();
    match unit.as_str() {
        "MB" => num / 1024.0,
        "GB" => num,
        "TB" => num * 1024.0,
        _ => 0.0,
    }
}

fn parse_memory_speed_mhz(s: &str) -> u32 {
    let mut parts = s.split_whitespace();
    let num = parts
        .next()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);
    let unit = parts.next().unwrap_or("MHz").to_uppercase();
    match unit.as_str() {
        "GHZ" => num * 1000,
        _ => num,
    }
}

fn read_drives() -> Vec<DriveSmartReport> {
    let json = run_cmd_lossy("system_profiler", &["SPNVMeDataType", "-json"]);
    let parsed: serde_json::Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let arr = match parsed.get("SPNVMeDataType").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return Vec::new(),
    };

    let mut out = Vec::new();
    for controller in arr {
        let drives = match controller.get("_items").and_then(|v| v.as_array()) {
            Some(d) => d,
            None => continue,
        };
        for drive in drives {
            let model = drive
                .get("device_model")
                .and_then(|v| v.as_str())
                .or_else(|| drive.get("_name").and_then(|v| v.as_str()))
                .unwrap_or("")
                .trim()
                .to_string();
            let size_bytes = drive
                .get("size_in_bytes")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let smart = drive
                .get("spnvme_smart_status")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let health_status = if smart.eq_ignore_ascii_case("Verified") {
                "OK".to_string()
            } else if smart.is_empty() {
                "Unknown".to_string()
            } else {
                "Failing".to_string()
            };

            out.push(DriveSmartReport {
                model,
                size_gb: (size_bytes / 1_000_000_000),
                health_status,
                temperature_c: None,
                power_on_hours: None,
                wear_percentage: None,
                read_errors_total: None,
            });
        }
    }
    out
}

fn read_battery() -> Option<BatteryReport> {
    let json = run_cmd_lossy("system_profiler", &["SPPowerDataType", "-json"]);
    let parsed: serde_json::Value = serde_json::from_str(&json).ok()?;
    let arr = parsed.get("SPPowerDataType").and_then(|v| v.as_array())?;

    for section in arr {
        let info = section
            .get("sppower_battery_health_info")
            .or_else(|| section.get("sppower_battery_charge_info"))
            .or_else(|| section.get("sppower_battery_info"));

        let cycle_count = info
            .and_then(|v| v.get("sppower_battery_cycle_count"))
            .and_then(|v| v.as_u64())
            .or_else(|| {
                section
                    .get("sppower_battery_health_info")
                    .and_then(|v| v.get("sppower_battery_cycle_count"))
                    .and_then(|v| v.as_u64())
            });
        let health = info
            .and_then(|v| v.get("sppower_battery_health_maximum_capacity"))
            .and_then(|v| v.as_str())
            .map(|s| s.trim_end_matches('%').trim().parse::<u32>().unwrap_or(0));
        let design = info
            .and_then(|v| v.get("sppower_battery_health_design_capacity"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.trim_end_matches(" mAh").trim().parse::<u32>().ok());
        let full = info
            .and_then(|v| v.get("sppower_battery_full_charge_capacity"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.trim_end_matches(" mAh").trim().parse::<u32>().ok());

        if cycle_count.is_some() || health.is_some() {
            return Some(BatteryReport {
                design_capacity_mwh: design.unwrap_or(0),
                full_charge_capacity_mwh: full.unwrap_or(0),
                cycle_count: cycle_count.unwrap_or(0) as u32,
                health_percent: health.unwrap_or(0),
            });
        }
    }
    None
}

fn read_security() -> SecurityReport {
    let firewall_enabled = read_firewall();
    SecurityReport {
        antivirus_name: None,
        antivirus_enabled: false,
        antivirus_up_to_date: false,
        firewall_enabled,
        bitlocker_status: read_filevault_label(),
        tpm_present: false,
        tpm_enabled: false,
    }
}

fn read_firewall() -> bool {
    let raw = run_cmd_lossy(
        "defaults",
        &["read", "/Library/Preferences/com.apple.alf", "globalstate"],
    );
    matches!(raw.trim(), "1" | "2")
}

fn read_filevault_label() -> String {
    let raw = run_cmd_lossy("fdesetup", &["status"]);
    if raw.to_lowercase().contains("filevault is on") {
        "Enabled".to_string()
    } else if raw.to_lowercase().contains("filevault is off") {
        "Disabled".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn read_software_count() -> u32 {
    let json = run_cmd_lossy("system_profiler", &["SPApplicationsDataType", "-json"]);
    let parsed: serde_json::Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    parsed
        .get("SPApplicationsDataType")
        .and_then(|v| v.as_array())
        .map(|a| a.len() as u32)
        .unwrap_or(0)
}

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

fn compute_grade(report: &ReportData) -> (String, u32) {
    let mut score: i32 = 100;

    for d in &report.drives {
        match d.health_status.as_str() {
            "Failing" => score -= 30,
            "Warning" => score -= 20,
            _ => {}
        }
    }

    if !read_sip_enabled() {
        score -= 15;
    }
    if !read_gatekeeper_enabled() {
        score -= 10;
    }
    if !read_filevault_enabled() {
        score -= 10;
    }
    if !report.security.firewall_enabled {
        score -= 5;
    }

    if let Some(b) = &report.battery {
        if b.cycle_count > 1000 || b.health_percent < 60 {
            score -= 10;
        }
    }

    if report.startup_enabled_count > 10 {
        let extra = (report.startup_enabled_count - 10) as i32;
        score -= extra.min(10);
    }

    let clamped = score.max(0) as u32;
    let grade = match clamped {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    (grade.to_string(), clamped)
}

fn chrono_like_now() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let days = (secs / 86400) as i64;
    let tod = (secs % 86400) as u32;
    let hour = tod / 3600;
    let min = (tod % 3600) / 60;
    let sec = tod % 60;

    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hour, min, sec
    )
}
