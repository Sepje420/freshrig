use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WingetSearchResult {
    pub name: String,
    pub id: String,
    pub version: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WingetPackageDetails {
    pub id: String,
    pub name: String,
    pub version: String,
    pub publisher: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

#[tauri::command]
pub async fn search_winget_packages(query: String) -> Result<Vec<WingetSearchResult>, String> {
    if query.trim().len() < 2 {
        return Ok(vec![]);
    }

    let sanitized = query.replace('"', "");
    let output = Command::new("cmd")
        .args([
            "/C",
            &format!(
                "chcp 65001 >nul && winget search \"{}\" --source winget --accept-source-agreements --disable-interactivity --count 20",
                sanitized
            ),
        ])
        .output()
        .map_err(|e| format!("Failed to run winget: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    parse_winget_search_output(&stdout)
}

fn parse_winget_search_output(output: &str) -> Result<Vec<WingetSearchResult>, String> {
    let lines: Vec<&str> = output.lines().collect();

    // Find the header line (starts with "Name")
    let header_idx = match lines.iter().position(|l| l.starts_with("Name")) {
        Some(idx) => idx,
        None => return Ok(vec![]),
    };

    let header = lines[header_idx];
    let separator_idx = header_idx + 1;

    // Find column positions from header
    let id_col = header.find("Id").ok_or("Id column not found")?;
    let ver_col = header.find("Version").ok_or("Version column not found")?;
    let src_col = header.find("Source");

    let mut results = Vec::new();

    for line in lines.iter().skip(separator_idx + 1) {
        let line = *line;
        if line.trim().is_empty() || line.contains("results match") {
            continue;
        }
        if line.len() < ver_col {
            continue;
        }

        let name = line[..id_col].trim().to_string();
        let id = line[id_col..ver_col].trim().to_string();
        let version = match src_col {
            Some(s) if line.len() >= s => line[ver_col..s].trim().to_string(),
            _ => line[ver_col..].trim().to_string(),
        };
        let source = src_col
            .filter(|&s| line.len() >= s)
            .map(|s| line[s..].trim().to_string())
            .unwrap_or_default();

        // Skip entries with truncated IDs (contain Unicode ellipsis)
        if id.contains('\u{2026}') || id.is_empty() {
            continue;
        }

        results.push(WingetSearchResult {
            name,
            id,
            version,
            source,
        });
    }

    Ok(results)
}

#[tauri::command]
pub async fn get_winget_package_info(package_id: String) -> Result<WingetPackageDetails, String> {
    let sanitized = package_id.replace('"', "");
    let output = Command::new("cmd")
        .args([
            "/C",
            &format!(
                "chcp 65001 >nul && winget show --id {} -e --accept-source-agreements --disable-interactivity",
                sanitized
            ),
        ])
        .output()
        .map_err(|e| format!("Failed to run winget show: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    parse_winget_show_output(&stdout)
}

fn parse_winget_show_output(output: &str) -> Result<WingetPackageDetails, String> {
    let mut details = WingetPackageDetails {
        id: String::new(),
        name: String::new(),
        version: String::new(),
        publisher: None,
        description: None,
        homepage: None,
        license: None,
    };

    for line in output.lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("Id:") {
            details.id = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("Name:") {
            details.name = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("Version:") {
            details.version = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("Publisher:") {
            details.publisher = Some(val.trim().to_string());
        } else if let Some(val) = line.strip_prefix("Description:") {
            details.description = Some(val.trim().to_string());
        } else if let Some(val) = line.strip_prefix("Homepage:") {
            details.homepage = Some(val.trim().to_string());
        } else if let Some(val) = line.strip_prefix("License:") {
            details.license = Some(val.trim().to_string());
        }
    }

    if details.id.is_empty() {
        return Err("Could not parse package details".to_string());
    }

    Ok(details)
}
