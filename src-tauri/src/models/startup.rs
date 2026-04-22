use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupEntry {
    pub id: String,
    pub name: String,
    pub command: String,
    pub source: StartupSource,
    pub scope: StartupScope,
    pub enabled: bool,
    pub publisher: Option<String>,
    pub impact: StartupImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StartupSource {
    RegistryRun,
    RegistryRunOnce,
    StartupFolder,
    TaskScheduler,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StartupScope {
    CurrentUser,
    AllUsers,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StartupImpact {
    High,
    Medium,
    Low,
    NotMeasured,
}
