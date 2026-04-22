// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
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
    pub install_action: DriverInstallAction,
    pub install_label: String,
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

/// Adjacently tagged so the JSON looks like `{"type":"Winget","value":"..."}`,
/// matching the discriminated union the frontend consumes.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum DriverInstallAction {
    /// Install via winget — value is the winget package id.
    Winget(String),
    /// Open a vendor download page in the browser — value is the URL.
    DirectDownload(String),
}
