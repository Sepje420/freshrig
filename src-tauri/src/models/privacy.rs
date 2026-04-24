use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacySetting {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: PrivacyCategory,
    pub risk: PrivacyRisk,
    pub current_value: bool,
    pub recommended: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyCategory {
    Telemetry,
    Permissions,
    Advertising,
    Activity,
    AiCopilot,
    Search,
    Suggestions,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyRisk {
    Recommended,
    Limited,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPermission {
    pub app_name: String,
    pub app_path: Option<String>,
    pub capability: String,
    pub allowed: bool,
    pub last_used: Option<String>,
    pub is_active_now: bool,
}
