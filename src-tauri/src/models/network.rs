// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    pub name: String,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiProfile {
    pub ssid: String,
    #[serde(default)]
    pub password: Option<String>,
    pub auth_type: String,
}
