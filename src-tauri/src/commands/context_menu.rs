// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
use crate::models::context_menu::ShellExtension;
use crate::util::silent_cmd;
use std::collections::{BTreeMap, HashSet};
use winreg::enums::*;
use winreg::RegKey;

const WIN11_CLASSIC_CLSID: &str = "{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}";
const BLOCKED_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked";
const HANDLER_ROOTS: &[&str] = &[
    r"*\shellex\ContextMenuHandlers",
    r"Directory\shellex\ContextMenuHandlers",
    r"Directory\Background\shellex\ContextMenuHandlers",
];

fn normalize_clsid(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Some handlers register by ProgID rather than CLSID — skip non-GUIDs
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

fn read_default_value(root: &RegKey, subkey: &str) -> Option<String> {
    root.open_subkey(subkey)
        .ok()?
        .get_value::<String, _>("")
        .ok()
}

fn read_blocked_set() -> HashSet<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut set = HashSet::new();
    if let Ok(key) = hklm.open_subkey(BLOCKED_KEY) {
        for (name, _) in key.enum_values().flatten() {
            if let Some(clsid) = normalize_clsid(&name) {
                set.insert(clsid);
            }
        }
    }
    set
}

#[tauri::command]
pub async fn get_classic_menu_status() -> Result<bool, String> {
    tokio::task::spawn_blocking(|| {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = format!(
            r"Software\Classes\CLSID\{}\InprocServer32",
            WIN11_CLASSIC_CLSID
        );
        Ok(hkcu.open_subkey(&path).is_ok())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

fn restart_explorer() {
    let _ = silent_cmd("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .output();
    let _ = silent_cmd("explorer.exe").spawn();
}

#[tauri::command]
pub async fn toggle_classic_menu(enable: bool) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let clsid_path = format!(r"Software\Classes\CLSID\{}", WIN11_CLASSIC_CLSID);
        let inproc_path = format!("{}\\InprocServer32", clsid_path);

        if enable {
            let (clsid_key, _) = hkcu
                .create_subkey(&clsid_path)
                .map_err(|e| format!("create CLSID key: {}", e))?;
            let (inproc_key, _) = clsid_key
                .create_subkey("InprocServer32")
                .map_err(|e| format!("create InprocServer32: {}", e))?;
            inproc_key
                .set_value("", &"")
                .map_err(|e| format!("set empty default: {}", e))?;
        } else {
            let _ = hkcu.delete_subkey_all(&inproc_path);
            let _ = hkcu.delete_subkey(&clsid_path);
        }
        restart_explorer();
        Ok(())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

fn resolve_extension(
    hkcr: &RegKey,
    handler_name: &str,
    clsid_value: &str,
    blocked: &HashSet<String>,
) -> Option<ShellExtension> {
    let clsid = normalize_clsid(clsid_value)?;

    let clsid_key_path = format!(r"CLSID\{}", clsid);
    let clsid_key = hkcr.open_subkey(&clsid_key_path).ok();

    let dll_path = clsid_key
        .as_ref()
        .and_then(|k| k.open_subkey("InprocServer32").ok())
        .and_then(|k| k.get_value::<String, _>("").ok())
        .unwrap_or_default();

    let friendly_name = clsid_key
        .as_ref()
        .and_then(|k| k.get_value::<String, _>("").ok())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| handler_name.trim_start_matches('{').to_string());

    let is_microsoft = {
        let lower = dll_path.to_ascii_lowercase();
        lower.contains(r"\windows\system32\")
            || lower.contains(r"\windows\syswow64\")
            || lower.contains(r"\windows\systemapps\")
            || lower.contains(r"\program files\windowsapps\microsoft.")
    };

    Some(ShellExtension {
        name: friendly_name,
        clsid: clsid.clone(),
        dll_path,
        company: None,
        is_blocked: blocked.contains(&clsid),
        is_microsoft,
    })
}

#[tauri::command]
pub async fn get_shell_extensions() -> Result<Vec<ShellExtension>, String> {
    tokio::task::spawn_blocking(|| {
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
        let blocked = read_blocked_set();

        // Deduplicate by CLSID — the same handler often appears under multiple roots.
        let mut seen: BTreeMap<String, ShellExtension> = BTreeMap::new();

        for root in HANDLER_ROOTS {
            let handlers = match hkcr.open_subkey(root) {
                Ok(k) => k,
                Err(_) => continue,
            };
            for name in handlers.enum_keys().flatten() {
                let subkey_path = format!("{}\\{}", root, name);
                let clsid_value = match read_default_value(&hkcr, &subkey_path) {
                    Some(v) if !v.trim().is_empty() => v,
                    _ => name.clone(),
                };
                if let Some(ext) = resolve_extension(&hkcr, &name, &clsid_value, &blocked) {
                    seen.entry(ext.clsid.clone()).or_insert(ext);
                }
            }
        }

        let mut out: Vec<ShellExtension> = seen.into_values().collect();
        out.sort_by(|a, b| {
            a.is_microsoft.cmp(&b.is_microsoft).then_with(|| {
                a.name
                    .to_ascii_lowercase()
                    .cmp(&b.name.to_ascii_lowercase())
            })
        });
        Ok(out)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub async fn toggle_shell_extension(clsid: String, block: bool) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let normalized =
            normalize_clsid(&clsid).ok_or_else(|| format!("Invalid CLSID: {}", clsid))?;
        let (key, _) = hklm
            .create_subkey(BLOCKED_KEY)
            .map_err(|e| format!("open Blocked key: {}", e))?;
        if block {
            key.set_value(&normalized, &"")
                .map_err(|e| format!("block {}: {}", normalized, e))?;
        } else {
            // delete_value may fail if not present — that's fine.
            let _ = key.delete_value(&normalized);
        }
        Ok(())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}
