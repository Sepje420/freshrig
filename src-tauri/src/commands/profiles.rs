use std::collections::HashMap;
use std::fs;
use std::io::{Read as IoRead, Write as IoWrite};
use std::path::PathBuf;

use base64::Engine;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use tauri_plugin_dialog::DialogExt;
use wmi::{COMLibrary, WMIConnection};

use crate::models::apps::AppEntry;
use crate::models::profiles::*;

fn profiles_dir() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA")
        .map_err(|_| "Could not read APPDATA environment variable".to_string())?;
    let dir = PathBuf::from(appdata)
        .join("com.freshrig.app")
        .join("profiles");
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create profiles directory: {}", e))?;
    }
    Ok(dir)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

#[tauri::command]
pub async fn save_profile(profile: RigProfile) -> Result<String, String> {
    let dir = profiles_dir()?;
    let filename = format!(
        "{}.freshrig.json",
        sanitize_filename(&profile.metadata.name)
    );
    let path = dir.join(&filename);

    let json = serde_json::to_string_pretty(&profile)
        .map_err(|e| format!("Failed to serialize profile: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("Failed to write profile: {}", e))?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn load_profile(file_path: String) -> Result<RigProfile, String> {
    let data =
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read profile: {}", e))?;

    let profile: RigProfile =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse profile: {}", e))?;

    if profile.config_version != 1 {
        return Err(format!(
            "Unsupported profile version: {}. Expected 1.",
            profile.config_version
        ));
    }

    Ok(profile)
}

#[tauri::command]
pub async fn list_profiles() -> Result<Vec<ProfileSummary>, String> {
    let dir = profiles_dir()?;

    let mut summaries: Vec<ProfileSummary> = Vec::new();

    let entries =
        fs::read_dir(&dir).map_err(|e| format!("Failed to read profiles directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json")
            && path
                .file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".freshrig.json"))
        {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(profile) = serde_json::from_str::<RigProfile>(&data) {
                    summaries.push(ProfileSummary {
                        file_path: path.to_string_lossy().to_string(),
                        name: profile.metadata.name,
                        description: profile.metadata.description,
                        app_count: profile.apps.len(),
                        created_at: profile.metadata.created_at,
                        updated_at: profile.metadata.updated_at,
                    });
                }
            }
        }
    }

    // Sort by updated_at descending
    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    Ok(summaries)
}

#[tauri::command]
pub async fn delete_profile(file_path: String) -> Result<(), String> {
    let path = PathBuf::from(&file_path);
    let dir = profiles_dir()?;

    // Verify path is inside profiles dir
    let canonical_path = path
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    let canonical_dir = dir
        .canonicalize()
        .map_err(|e| format!("Invalid profiles dir: {}", e))?;

    if !canonical_path.starts_with(&canonical_dir) {
        return Err("Cannot delete files outside the profiles directory".to_string());
    }

    fs::remove_file(&path).map_err(|e| format!("Failed to delete profile: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn export_profile_to_file(
    app_handle: tauri::AppHandle,
    profile: RigProfile,
) -> Result<String, String> {
    let json = serde_json::to_string_pretty(&profile)
        .map_err(|e| format!("Failed to serialize profile: {}", e))?;

    let filename = format!(
        "{}.freshrig.json",
        sanitize_filename(&profile.metadata.name)
    );

    let file_path = app_handle
        .dialog()
        .file()
        .set_file_name(&filename)
        .add_filter("FreshRig Profile", &["freshrig.json", "json"])
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            fs::write(&path_str, json).map_err(|e| format!("Failed to write profile: {}", e))?;
            Ok(path_str)
        }
        None => Err("Save cancelled".to_string()),
    }
}

#[tauri::command]
pub async fn import_profile_from_file(app_handle: tauri::AppHandle) -> Result<RigProfile, String> {
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("FreshRig Profile", &["freshrig.json", "json"])
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            let data =
                fs::read_to_string(&path_str).map_err(|e| format!("Failed to read file: {}", e))?;
            let profile: RigProfile = serde_json::from_str(&data)
                .map_err(|e| format!("Failed to parse profile: {}", e))?;

            if profile.config_version != 1 {
                return Err(format!(
                    "Unsupported profile version: {}",
                    profile.config_version
                ));
            }

            Ok(profile)
        }
        None => Err("Import cancelled".to_string()),
    }
}

#[tauri::command]
pub async fn export_profile_as_text(
    profile: RigProfile,
    catalog: Vec<AppEntry>,
) -> Result<String, String> {
    let name_map: HashMap<String, &AppEntry> = catalog.iter().map(|a| (a.id.clone(), a)).collect();

    // Group apps by category
    let mut by_category: HashMap<String, Vec<String>> = HashMap::new();
    for app_id in &profile.apps {
        if let Some(entry) = name_map.get(app_id) {
            let cat = format!("{:?}", entry.category);
            by_category.entry(cat).or_default().push(entry.name.clone());
        } else {
            by_category
                .entry("Other".to_string())
                .or_default()
                .push(app_id.clone());
        }
    }

    let mut md = String::new();
    md.push_str(&format!(
        "## My FreshRig Setup: {}\n",
        profile.metadata.name
    ));
    if let Some(desc) = &profile.metadata.description {
        md.push_str(&format!("{}\n", desc));
    }
    md.push('\n');
    md.push_str("| Category | Apps |\n");
    md.push_str("|----------|------|\n");

    // Sort categories for consistent output
    let mut cats: Vec<_> = by_category.keys().cloned().collect();
    cats.sort();
    for cat in cats {
        if let Some(apps) = by_category.get(&cat) {
            md.push_str(&format!("| {} | {} |\n", cat, apps.join(", ")));
        }
    }

    md.push_str(&format!(
        "\n> Created with [FreshRig](https://freshrig.app) — {} apps, 1-click install\n",
        profile.apps.len()
    ));

    Ok(md)
}

#[tauri::command]
pub async fn compress_profile(profile: RigProfile) -> Result<String, String> {
    let json =
        serde_json::to_vec(&profile).map_err(|e| format!("Failed to serialize profile: {}", e))?;

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(&json)
        .map_err(|e| format!("Compression error: {}", e))?;
    let compressed = encoder
        .finish()
        .map_err(|e| format!("Compression finish error: {}", e))?;

    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&compressed);

    Ok(encoded)
}

#[tauri::command]
pub async fn decompress_profile(encoded: String) -> Result<RigProfile, String> {
    let compressed = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(&encoded)
        .map_err(|e| format!("Invalid share code (base64 error): {}", e))?;

    let mut decoder = DeflateDecoder::new(&compressed[..]);
    let mut json = Vec::new();
    decoder
        .read_to_end(&mut json)
        .map_err(|e| format!("Invalid share code (decompression error): {}", e))?;

    let profile: RigProfile = serde_json::from_slice(&json)
        .map_err(|e| format!("Invalid share code (parse error): {}", e))?;

    if profile.config_version != 1 {
        return Err(format!(
            "Unsupported profile version: {}",
            profile.config_version
        ));
    }

    Ok(profile)
}

#[tauri::command]
pub async fn get_current_hardware_snapshot() -> Result<SourceHardware, String> {
    tokio::task::spawn_blocking(|| {
        let com = COMLibrary::new().map_err(|e| format!("COM error: {}", e))?;
        let wmi = WMIConnection::new(com).map_err(|e| format!("WMI error: {}", e))?;

        let cpu: Option<String> = wmi
            .raw_query::<HashMap<String, wmi::Variant>>("SELECT Name FROM Win32_Processor")
            .ok()
            .and_then(|r| r.first().cloned())
            .and_then(|r| match r.get("Name") {
                Some(wmi::Variant::String(s)) => Some(s.trim().to_string()),
                _ => None,
            });

        let gpu: Option<String> = wmi
            .raw_query::<HashMap<String, wmi::Variant>>("SELECT Name FROM Win32_VideoController")
            .ok()
            .and_then(|r| r.first().cloned())
            .and_then(|r| match r.get("Name") {
                Some(wmi::Variant::String(s)) => Some(s.clone()),
                _ => None,
            });

        let ram_gb: Option<f64> = wmi
            .raw_query::<HashMap<String, wmi::Variant>>(
                "SELECT TotalPhysicalMemory FROM Win32_ComputerSystem",
            )
            .ok()
            .and_then(|r| r.first().cloned())
            .and_then(|r| match r.get("TotalPhysicalMemory") {
                Some(wmi::Variant::String(s)) => s.parse::<u64>().ok(),
                Some(wmi::Variant::UI8(n)) => Some(*n),
                _ => None,
            })
            .map(|b| b as f64 / (1024.0 * 1024.0 * 1024.0));

        let os: Option<String> = wmi
            .raw_query::<HashMap<String, wmi::Variant>>("SELECT Caption FROM Win32_OperatingSystem")
            .ok()
            .and_then(|r| r.first().cloned())
            .and_then(|r| match r.get("Caption") {
                Some(wmi::Variant::String(s)) => Some(s.clone()),
                _ => None,
            });

        Ok(SourceHardware {
            cpu,
            gpu,
            ram_gb,
            os,
        })
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
