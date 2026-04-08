use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriverRecommendation {
    pub device_name: String,
    pub category: DriverCategory,
    pub vendor: String,
    pub current_version: Option<String>,
    pub current_date: Option<String>,
    pub download_url: String,
    pub download_page: String,
    pub status: DriverStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DriverCategory {
    Gpu,
    Chipset,
    Network,
    Audio,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DriverStatus {
    UpToDate,
    UpdateAvailable,
    Missing,
    Unknown,
}
