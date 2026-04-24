use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub os_build: String,
    pub hostname: String,
    pub architecture: String,
    pub uptime_seconds: u64,
    pub desktop_environment: Option<String>,
    pub distro_id: Option<String>,
    pub distro_family: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub name: String,
    pub cores: u32,
    pub threads: u32,
    pub base_clock_mhz: Option<f64>,
    pub temperature_celsius: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub vram_mb: u64,
    pub driver_version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub name: String,
    pub model: String,
    pub size_gb: f64,
    pub media_type: String,
    pub health_status: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total_gb: f64,
    pub used_gb: f64,
    pub slots: Vec<MemorySlot>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemorySlot {
    pub capacity_gb: f64,
    pub speed_mhz: Option<u32>,
    pub form_factor: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    pub adapter_name: String,
    pub mac_address: Option<String>,
    pub speed_mbps: Option<u64>,
    pub is_connected: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatteryInfo {
    pub present: bool,
    pub health_percent: Option<f64>,
    pub cycle_count: Option<u32>,
    pub design_capacity_mwh: Option<u64>,
    pub full_charge_capacity_mwh: Option<u64>,
}
