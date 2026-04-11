use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomAppEntry {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub download_url: String,
    pub installer_type: InstallerType,
    pub silent_args: String,
    pub expected_hash: Option<String>,
    pub created_at: String,
    pub last_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InstallerType {
    Nsis,
    InnoSetup,
    Msi,
    Exe,
    Unknown,
}

#[allow(dead_code)]
impl InstallerType {
    pub fn default_args(&self) -> &str {
        match self {
            Self::Nsis => "/S",
            Self::InnoSetup => "/VERYSILENT /SUPPRESSMSGBOXES /NORESTART",
            Self::Msi => "/qn /norestart",
            Self::Exe | Self::Unknown => "",
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        let lower = filename.to_lowercase();
        if lower.ends_with(".msi") {
            return Self::Msi;
        }
        if lower.contains("setup") || lower.contains("install") {
            return Self::Nsis;
        }
        Self::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub filename: String,
}
