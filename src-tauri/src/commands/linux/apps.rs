//! Linux application install orchestration — distro-family dispatch over
//! apt / dnf / pacman / zypper, with Flatpak fallback.

use std::process::Command;

use tauri::Emitter;

use crate::commands::linux::app_catalog::{find_name, find_package, linux_app_catalog};
use crate::commands::linux::util::{distro_family, is_root, run_cmd, which};
use crate::models::apps::*;

#[tauri::command]
pub async fn get_app_catalog() -> Result<Vec<AppEntry>, String> {
    Ok(linux_app_catalog())
}

#[tauri::command]
pub async fn get_free_disk_space_gb() -> Result<f64, String> {
    tokio::task::spawn_blocking(|| {
        let stat = nix::sys::statvfs::statvfs("/").map_err(|e| format!("statvfs failed: {}", e))?;
        let frsize = stat.fragment_size() as u64;
        let bavail = stat.blocks_available() as u64;
        let bytes = frsize.saturating_mul(bavail);
        Ok((bytes as f64) / 1_000_000_000.0)
    })
    .await
    .map_err(|e| format!("disk task failed: {}", e))?
}

#[tauri::command]
pub async fn check_network_connectivity() -> Result<bool, String> {
    tokio::task::spawn_blocking(|| {
        let output = Command::new("ping")
            .args(["-c", "1", "-W", "3", "1.1.1.1"])
            .output();
        Ok(match output {
            Ok(o) => o.status.success(),
            Err(_) => false,
        })
    })
    .await
    .map_err(|e| format!("ping task failed: {}", e))?
}

#[tauri::command]
pub async fn check_winget_available() -> Result<bool, String> {
    // On Linux we treat the native package manager as the "winget equivalent"
    // and report availability if at least one is on PATH.
    Ok(which("apt-get") || which("dnf") || which("pacman") || which("zypper") || which("flatpak"))
}

#[tauri::command]
pub async fn install_apps(
    app_handle: tauri::AppHandle,
    app_ids: Vec<String>,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let family = distro_family();

        for app_id in &app_ids {
            let app_name = find_name(app_id).unwrap_or(app_id.as_str()).to_string();

            emit(
                &app_handle,
                app_id,
                &app_name,
                InstallStatus::Installing,
                &format!("Installing {}…", app_name),
            );

            let pkg = match find_package(app_id) {
                Some(p) => p,
                None => {
                    emit(
                        &app_handle,
                        app_id,
                        &app_name,
                        InstallStatus::Skipped,
                        "No package mapping available on Linux.",
                    );
                    continue;
                }
            };

            let (program, args) = match build_install_cmd(&family, pkg) {
                Some(cmd) => cmd,
                None => {
                    emit(
                        &app_handle,
                        app_id,
                        &app_name,
                        InstallStatus::Skipped,
                        "No install target for this distro.",
                    );
                    continue;
                }
            };

            let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            match run_cmd(&program, &arg_refs) {
                Ok(_) => {
                    emit(
                        &app_handle,
                        app_id,
                        &app_name,
                        InstallStatus::Completed,
                        &format!("{} installed.", app_name),
                    );
                }
                Err(err) => {
                    emit(&app_handle, app_id, &app_name, InstallStatus::Failed, &err);
                }
            }
        }

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("install task failed: {}", e))?
}

fn emit(
    app_handle: &tauri::AppHandle,
    app_id: &str,
    app_name: &str,
    status: InstallStatus,
    message: &str,
) {
    let _ = app_handle.emit(
        "install-progress",
        InstallProgress {
            app_id: app_id.to_string(),
            app_name: app_name.to_string(),
            status,
            message: message.to_string(),
        },
    );
}

/// Build the shell command to install one package for the current distro.
/// Returns `(program, args)` where the leading program may be `pkexec` if the
/// process isn't already root.
fn build_install_cmd(
    family: &str,
    pkg: crate::commands::linux::app_catalog::LinuxPackage,
) -> Option<(String, Vec<String>)> {
    let native = match family {
        "debian" => pkg.apt.map(|p| {
            (
                "apt-get",
                vec!["install", "-y", p]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )
        }),
        "rhel" => pkg.dnf.map(|p| {
            (
                "dnf",
                vec!["install", "-y", p]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )
        }),
        "arch" => pkg.pacman.map(|p| {
            (
                "pacman",
                vec!["-S", "--noconfirm", "--needed", p]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )
        }),
        "suse" => pkg.zypper.map(|p| {
            (
                "zypper",
                vec!["--non-interactive", "install", p]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )
        }),
        _ => None,
    };

    if let Some((program, args)) = native {
        return Some(wrap_privilege(program, args));
    }

    // Fallback to Flatpak.
    if let Some(reference) = pkg.flatpak {
        if which("flatpak") {
            return Some((
                "flatpak".to_string(),
                vec![
                    "install".to_string(),
                    "-y".to_string(),
                    "--noninteractive".to_string(),
                    "flathub".to_string(),
                    reference.to_string(),
                ],
            ));
        }
    }

    // Last resort: snap.
    if let Some(s) = pkg.snap {
        if which("snap") {
            let args = vec!["install".to_string(), s.to_string()];
            return Some(wrap_privilege("snap", args));
        }
    }

    None
}

/// Prepend `pkexec` + inline env scrubbing when not already root.
fn wrap_privilege(program: &str, args: Vec<String>) -> (String, Vec<String>) {
    if is_root() {
        return (program.to_string(), args);
    }

    let mut all = Vec::with_capacity(args.len() + 3);
    // Ensure non-interactive behavior for apt and friends.
    all.push("env".to_string());
    all.push("DEBIAN_FRONTEND=noninteractive".to_string());
    all.push(program.to_string());
    all.extend(args);

    ("pkexec".to_string(), all)
}
