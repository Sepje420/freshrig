// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellExtension {
    pub name: String,
    pub clsid: String,
    pub dll_path: String,
    #[serde(default)]
    pub company: Option<String>,
    pub is_blocked: bool,
    pub is_microsoft: bool,
}
