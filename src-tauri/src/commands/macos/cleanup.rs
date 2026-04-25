//! macOS disk cleanup — scans `~/Library/Caches`, `~/Library/Logs`, Xcode
//! DerivedData, the Homebrew cache, the user Trash, the iOS Simulator
//! cache, old downloads, and `/private/var/log`. Mirrors the Windows +
//! Linux command surface so the frontend works unchanged.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use jwalk::WalkDir;
use tauri::{AppHandle, Emitter};

use crate::commands::macos::util::{brew_path, home_dir, run_elevated};
use crate::models::cleanup::*;

struct CategorySpec {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    risk: CleanupRisk,
    enabled_by_default: bool,
    kind: CleanupKind,
}

enum CleanupKind {
    /// Walk + delete every file under the given paths.
    UserFiles {
        paths: Vec<PathBuf>,
        /// Top-level subdir names to exclude.
        exclude_top_level: &'static [&'static str],
    },
    /// `brew cleanup -s --prune=all`.
    BrewCleanup,
    /// XDG-style ~/.Trash dump.
    Trash,
    /// Files older than N days under a directory.
    OldFiles { dir: PathBuf, max_age_days: u64 },
    /// `/private/var/log` — needs root.
    SystemLogs,
}

fn specs() -> Vec<CategorySpec> {
    let home = home_dir().unwrap_or_else(|| PathBuf::from("/Users"));

    let user_caches = home.join("Library/Caches");
    let user_logs = home.join("Library/Logs");
    let xcode_derived = home.join("Library/Developer/Xcode/DerivedData");
    let ios_sim_caches = home.join("Library/Developer/CoreSimulator/Caches");
    let downloads = home.join("Downloads");

    vec![
        CategorySpec {
            id: "user_caches",
            name: "User caches",
            description: "Cached files under ~/Library/Caches (excluding Homebrew).".into(),
            risk: CleanupRisk::Safe,
            enabled_by_default: true,
            kind: CleanupKind::UserFiles {
                paths: vec![user_caches.clone()],
                exclude_top_level: &["Homebrew"],
            },
        },
        CategorySpec {
            id: "user_logs",
            name: "User logs",
            description: "App logs under ~/Library/Logs.".into(),
            risk: CleanupRisk::Safe,
            enabled_by_default: true,
            kind: CleanupKind::UserFiles {
                paths: vec![user_logs],
                exclude_top_level: &[],
            },
        },
        CategorySpec {
            id: "xcode_derived_data",
            name: "Xcode DerivedData",
            description: "Build artifacts under ~/Library/Developer/Xcode/DerivedData.".into(),
            risk: CleanupRisk::Safe,
            enabled_by_default: true,
            kind: CleanupKind::UserFiles {
                paths: vec![xcode_derived],
                exclude_top_level: &[],
            },
        },
        CategorySpec {
            id: "homebrew_cache",
            name: "Homebrew cache",
            description: "Cached bottles and formulae managed by `brew cleanup`.".into(),
            risk: CleanupRisk::Safe,
            enabled_by_default: true,
            kind: CleanupKind::BrewCleanup,
        },
        CategorySpec {
            id: "trash",
            name: "Trash",
            description: "Files in ~/.Trash.".into(),
            risk: CleanupRisk::Safe,
            enabled_by_default: true,
            kind: CleanupKind::Trash,
        },
        CategorySpec {
            id: "ios_simulator",
            name: "iOS Simulator cache",
            description: "~/Library/Developer/CoreSimulator/Caches.".into(),
            risk: CleanupRisk::Moderate,
            enabled_by_default: false,
            kind: CleanupKind::UserFiles {
                paths: vec![ios_sim_caches],
                exclude_top_level: &[],
            },
        },
        CategorySpec {
            id: "old_downloads",
            name: "Old downloads",
            description: "Files in ~/Downloads older than 90 days.".into(),
            risk: CleanupRisk::Expert,
            enabled_by_default: false,
            kind: CleanupKind::OldFiles {
                dir: downloads,
                max_age_days: 90,
            },
        },
        CategorySpec {
            id: "system_logs",
            name: "System logs",
            description: "/private/var/log archives. Requires admin authentication.".into(),
            risk: CleanupRisk::Expert,
            enabled_by_default: false,
            kind: CleanupKind::SystemLogs,
        },
    ]
}

#[tauri::command]
pub async fn scan_cleanup(app_handle: AppHandle) -> Result<Vec<CleanupCategory>, String> {
    tokio::task::spawn_blocking(move || {
        let mut out = Vec::new();
        for spec in specs() {
            let (file_count, total_bytes, paths_reported) = scan_spec(&spec);

            let category = CleanupCategory {
                id: spec.id.to_string(),
                name: spec.name.to_string(),
                description: spec.description.to_string(),
                risk: spec.risk.clone(),
                file_count,
                total_bytes,
                paths: paths_reported,
                enabled_by_default: spec.enabled_by_default,
            };

            let _ = app_handle.emit(
                "cleanup-scan-progress",
                CleanupScanProgress {
                    category_id: category.id.clone(),
                    file_count,
                    total_bytes,
                },
            );

            out.push(category);
        }
        Ok(out)
    })
    .await
    .map_err(|e| format!("scan task failed: {}", e))?
}

#[tauri::command]
pub async fn run_cleanup(
    app_handle: AppHandle,
    category_ids: Vec<String>,
) -> Result<Vec<CleanupResult>, String> {
    tokio::task::spawn_blocking(move || {
        let mut results = Vec::new();
        for spec in specs() {
            if !category_ids.iter().any(|id| id == spec.id) {
                continue;
            }
            let (files_deleted, bytes_freed, errors) = run_spec(&spec);

            let _ = app_handle.emit(
                "cleanup-progress",
                CleanupProgress {
                    category_id: spec.id.to_string(),
                    files_deleted,
                    bytes_freed,
                },
            );

            results.push(CleanupResult {
                category_id: spec.id.to_string(),
                files_deleted,
                bytes_freed,
                errors,
            });
        }
        Ok(results)
    })
    .await
    .map_err(|e| format!("cleanup task failed: {}", e))?
}

// ---- Scan ----

fn scan_spec(spec: &CategorySpec) -> (u64, u64, Vec<String>) {
    match &spec.kind {
        CleanupKind::UserFiles {
            paths,
            exclude_top_level,
        } => {
            let mut count = 0u64;
            let mut bytes = 0u64;
            let mut reported = Vec::new();
            for base in paths {
                if !base.exists() {
                    continue;
                }
                reported.push(base.to_string_lossy().to_string());
                let (c, b) = walk_bytes(base, exclude_top_level);
                count += c;
                bytes += b;
            }
            (count, bytes, reported)
        }
        CleanupKind::BrewCleanup => {
            // Scan the cache dir to give a size estimate. The actual cleanup
            // shells out to `brew cleanup` later.
            let dir = home_dir()
                .map(|h| h.join("Library/Caches/Homebrew"))
                .unwrap_or_default();
            if !dir.exists() {
                return (0, 0, Vec::new());
            }
            let (c, b) = walk_bytes(&dir, &[]);
            (c, b, vec![dir.to_string_lossy().to_string()])
        }
        CleanupKind::Trash => {
            let trash = home_dir().map(|h| h.join(".Trash")).unwrap_or_default();
            if !trash.exists() {
                return (0, 0, Vec::new());
            }
            let (c, b) = walk_bytes(&trash, &[]);
            (c, b, vec![trash.to_string_lossy().to_string()])
        }
        CleanupKind::OldFiles { dir, max_age_days } => {
            if !dir.exists() {
                return (0, 0, Vec::new());
            }
            let (c, b) = walk_old_files(dir, *max_age_days);
            (c, b, vec![dir.to_string_lossy().to_string()])
        }
        CleanupKind::SystemLogs => {
            let dir = Path::new("/private/var/log");
            if !dir.exists() {
                return (0, 0, Vec::new());
            }
            // Counting only top-level rotated logs (.gz/.0).
            let mut count = 0u64;
            let mut bytes = 0u64;
            if let Ok(rd) = fs::read_dir(dir) {
                for entry in rd.flatten() {
                    let path = entry.path();
                    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                        continue;
                    };
                    if name.ends_with(".gz") || name.ends_with(".0") || name.ends_with(".old") {
                        if let Ok(meta) = entry.metadata() {
                            count += 1;
                            bytes += meta.len();
                        }
                    }
                }
            }
            (count, bytes, vec!["/private/var/log/*.gz".into()])
        }
    }
}

fn walk_bytes(root: &Path, exclude_top_level: &[&str]) -> (u64, u64) {
    let excluded: std::collections::HashSet<&str> = exclude_top_level.iter().copied().collect();
    let root_components = root.components().count();

    let mut count = 0u64;
    let mut bytes = 0u64;
    for entry in WalkDir::new(root).follow_links(false).into_iter().flatten() {
        let path = entry.path();
        if !excluded.is_empty() {
            if let Some(top) = path
                .components()
                .nth(root_components)
                .and_then(|c| c.as_os_str().to_str())
            {
                if excluded.contains(top) {
                    continue;
                }
            }
        }
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                count += 1;
                bytes += meta.len();
            }
        }
    }
    (count, bytes)
}

fn walk_old_files(root: &Path, max_age_days: u64) -> (u64, u64) {
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(max_age_days * 86_400))
        .unwrap_or(SystemTime::UNIX_EPOCH);

    let mut count = 0u64;
    let mut bytes = 0u64;
    for entry in WalkDir::new(root).follow_links(false).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        if modified < cutoff {
            count += 1;
            bytes += meta.len();
        }
    }
    (count, bytes)
}

// ---- Run ----

fn run_spec(spec: &CategorySpec) -> (u64, u64, Vec<String>) {
    match &spec.kind {
        CleanupKind::UserFiles {
            paths,
            exclude_top_level,
        } => delete_trees(paths, exclude_top_level),
        CleanupKind::BrewCleanup => run_brew_cleanup(),
        CleanupKind::Trash => {
            let trash = home_dir().map(|h| h.join(".Trash")).unwrap_or_default();
            delete_trees(&[trash], &[])
        }
        CleanupKind::OldFiles { dir, max_age_days } => delete_old_files(dir, *max_age_days),
        CleanupKind::SystemLogs => run_system_logs(),
    }
}

fn delete_trees(paths: &[PathBuf], exclude_top_level: &[&str]) -> (u64, u64, Vec<String>) {
    let mut files_deleted = 0u64;
    let mut bytes_freed = 0u64;
    let mut errors = Vec::new();

    let excluded: std::collections::HashSet<&str> = exclude_top_level.iter().copied().collect();

    for base in paths {
        if !base.exists() {
            continue;
        }
        let root_components = base.components().count();

        for entry in WalkDir::new(base).follow_links(false).into_iter().flatten() {
            let path = entry.path();
            if !excluded.is_empty() {
                if let Some(top) = path
                    .components()
                    .nth(root_components)
                    .and_then(|c| c.as_os_str().to_str())
                {
                    if excluded.contains(top) {
                        continue;
                    }
                }
            }
            if !entry.file_type().is_file() {
                continue;
            }
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            match fs::remove_file(&path) {
                Ok(_) => {
                    files_deleted += 1;
                    bytes_freed += size;
                }
                Err(e) => errors.push(format!("{}: {}", path.display(), e)),
            }
        }
    }

    (files_deleted, bytes_freed, errors)
}

fn run_brew_cleanup() -> (u64, u64, Vec<String>) {
    let Some(brew) = brew_path() else {
        return (0, 0, vec!["Homebrew not installed".to_string()]);
    };
    let output = std::process::Command::new(brew)
        .args(["cleanup", "-s", "--prune=all"])
        .env("NONINTERACTIVE", "1")
        .env("HOMEBREW_NO_AUTO_UPDATE", "1")
        .output();
    match output {
        Ok(o) if o.status.success() => (0, 0, Vec::new()),
        Ok(o) => (
            0,
            0,
            vec![String::from_utf8_lossy(&o.stderr).trim().to_string()],
        ),
        Err(e) => (0, 0, vec![format!("brew cleanup failed: {}", e)]),
    }
}

fn delete_old_files(root: &Path, max_age_days: u64) -> (u64, u64, Vec<String>) {
    if !root.exists() {
        return (0, 0, Vec::new());
    }
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(max_age_days * 86_400))
        .unwrap_or(SystemTime::UNIX_EPOCH);

    let mut count = 0u64;
    let mut bytes = 0u64;
    let mut errors = Vec::new();
    for entry in WalkDir::new(root).follow_links(false).into_iter().flatten() {
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        if modified >= cutoff {
            continue;
        }
        let size = meta.len();
        match fs::remove_file(&path) {
            Ok(_) => {
                count += 1;
                bytes += size;
            }
            Err(e) => errors.push(format!("{}: {}", path.display(), e)),
        }
    }
    (count, bytes, errors)
}

fn run_system_logs() -> (u64, u64, Vec<String>) {
    // Best-effort: ask the user for elevation, run a single shell pipe that
    // deletes rotated logs and prints how many bytes were freed.
    let cmd = "find /private/var/log -type f \\( -name '*.gz' -o -name '*.old' -o -name '*.0' \\) -delete -print";
    match run_elevated(cmd) {
        Ok(out) => {
            let count = out.lines().filter(|l| !l.trim().is_empty()).count() as u64;
            // We don't have per-file sizes any more; report 0 bytes_freed
            // but include the file list as informational paths via errors=[].
            (count, 0, Vec::new())
        }
        Err(e) => (0, 0, vec![format!("system logs: {}", e)]),
    }
}
