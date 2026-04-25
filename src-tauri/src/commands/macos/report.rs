//! macOS health report stub. `ReportData` and its sub-structs are duplicated
//! verbatim from `commands::linux::report` (same `#[serde(rename_all =
//! "camelCase")]` derives) so the frontend consumes identical JSON on every
//! OS. The eventual implementation will pull data from `system_profiler
//! SPHardwareDataType -json`, `pmset -g batt`, `diskutil info -plist`,
//! `csrutil status`, and `softwareupdate -l`.

#![allow(dead_code)]

use serde::Serialize;

use crate::commands::macos::util::STUB_ERR;

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

#[tauri::command]
pub async fn generate_health_report(_app_version: String) -> Result<ReportData, String> {
    Err(STUB_ERR.into())
}
