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
    #[serde(default)]
    pub winget_id: Option<String>,
    #[serde(default = "default_install_action")]
    pub install_action: DriverInstallAction,
}

fn default_install_action() -> DriverInstallAction {
    DriverInstallAction::OpenUrl
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DriverInstallAction {
    Winget,
    OpenUrl,
}
