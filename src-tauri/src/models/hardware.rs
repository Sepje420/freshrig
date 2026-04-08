use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub hostname: String,
    pub os_version: String,
    pub os_build: String,
    pub architecture: String,
    pub total_ram_gb: f64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub name: String,
    pub manufacturer: String,
    pub cores: u32,
    pub threads: u32,
    pub max_clock_mhz: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    pub name: String,
    pub manufacturer: String,
    pub driver_version: String,
    pub driver_date: String,
    pub vram_mb: u64,
    pub pnp_device_id: String,
    pub status: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub model: String,
    pub size_gb: f64,
    pub media_type: String,
    pub interface_type: String,
    pub serial_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAdapter {
    pub name: String,
    pub manufacturer: String,
    pub mac_address: String,
    pub connection_status: String,
    pub speed_mbps: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
    pub name: String,
    pub manufacturer: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MotherboardInfo {
    pub manufacturer: String,
    pub product: String,
    pub serial_number: String,
    pub bios_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HardwareSummary {
    pub system: SystemInfo,
    pub cpu: CpuInfo,
    pub gpus: Vec<GpuInfo>,
    pub disks: Vec<DiskInfo>,
    pub network_adapters: Vec<NetworkAdapter>,
    pub audio_devices: Vec<AudioDevice>,
    pub motherboard: MotherboardInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriverIssue {
    pub device_name: String,
    pub device_id: String,
    pub hardware_id: Vec<String>,
    pub error_code: u16,
    pub error_description: String,
}
