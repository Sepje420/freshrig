use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TweakTier {
    Safe,
    Moderate,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TweakCategory {
    Privacy,
    Bloatware,
    Performance,
    Appearance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TweakType {
    RegistrySet,
    AppxRemove,
    ServiceDisable,
    ScheduledTask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebloatTweak {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: TweakTier,
    pub category: TweakCategory,
    pub tweak_type: TweakType,
    pub is_applied: bool,
    pub is_reversible: bool,
    pub warning: Option<String>,
    #[serde(default)]
    pub min_windows_build: Option<u32>,
    #[serde(default)]
    pub incompatible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebloatResult {
    pub tweak_id: String,
    pub success: bool,
    pub message: String,
}
