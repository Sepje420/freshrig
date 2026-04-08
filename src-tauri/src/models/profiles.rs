use serde::{Deserialize, Serialize};

use crate::models::apps::AppCategory;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RigProfile {
    pub config_version: u32,
    pub metadata: ProfileMetadata,
    pub apps: Vec<String>,
    pub categories: Vec<AppCategory>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileMetadata {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub app_version: String,
    #[serde(default)]
    pub source_hardware: Option<SourceHardware>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceHardware {
    pub cpu: Option<String>,
    pub gpu: Option<String>,
    pub ram_gb: Option<f64>,
    pub os: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSummary {
    pub file_path: String,
    pub name: String,
    pub description: Option<String>,
    pub app_count: usize,
    pub created_at: String,
    pub updated_at: String,
}
