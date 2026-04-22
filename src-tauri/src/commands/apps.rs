use tauri::Emitter;

use crate::data::app_catalog;
use crate::models::apps::*;
use crate::util::silent_cmd;

#[tauri::command]
pub async fn get_app_catalog() -> Result<Vec<AppEntry>, String> {
    Ok(app_catalog::get_default_catalog())
}

#[tauri::command]
pub async fn get_free_disk_space_gb() -> Result<f64, String> {
    let output = silent_cmd("powershell")
        .args(["-NoProfile", "-Command", "(Get-PSDrive C).Free / 1GB"])
        .output()
        .map_err(|e| format!("Failed to check disk space: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse disk space: {}", e))
}

#[tauri::command]
pub async fn check_network_connectivity() -> Result<bool, String> {
    let output = silent_cmd("cmd")
        .args(["/C", "ping -n 1 -w 3000 1.1.1.1"])
        .output();
    match output {
        Ok(result) => Ok(result.status.success()),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn check_winget_available() -> Result<bool, String> {
    let result = silent_cmd("cmd")
        .args(["/C", "chcp 65001 >nul && winget --version"])
        .output();

    match result {
        Ok(output) => Ok(output.status.success()),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn install_apps(
    app_handle: tauri::AppHandle,
    app_ids: Vec<String>,
) -> Result<(), String> {
    let catalog = app_catalog::get_default_catalog();

    for app_id in &app_ids {
        let app_name = catalog
            .iter()
            .find(|a| &a.id == app_id)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| app_id.clone());

        // Emit: Installing
        let _ = app_handle.emit(
            "install-progress",
            InstallProgress {
                app_id: app_id.clone(),
                app_name: app_name.clone(),
                status: InstallStatus::Installing,
                message: format!("Installing {}...", app_name),
            },
        );

        let output = silent_cmd("cmd")
            .args([
                "/C",
                &format!(
                    "chcp 65001 >nul && winget install --id {} --silent --accept-package-agreements --accept-source-agreements",
                    app_id
                ),
            ])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    let _ = app_handle.emit(
                        "install-progress",
                        InstallProgress {
                            app_id: app_id.clone(),
                            app_name: app_name.clone(),
                            status: InstallStatus::Completed,
                            message: format!("{} installed successfully", app_name),
                        },
                    );
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    // winget often writes errors to stdout
                    let error_msg = if !stderr.is_empty() {
                        stderr.to_string()
                    } else if !stdout.is_empty() {
                        // Extract last meaningful line
                        stdout
                            .lines()
                            .rev()
                            .find(|l| !l.trim().is_empty())
                            .unwrap_or("Unknown error")
                            .to_string()
                    } else {
                        "Installation failed with no output".to_string()
                    };

                    let _ = app_handle.emit(
                        "install-progress",
                        InstallProgress {
                            app_id: app_id.clone(),
                            app_name: app_name.clone(),
                            status: InstallStatus::Failed,
                            message: error_msg,
                        },
                    );
                }
            }
            Err(e) => {
                let _ = app_handle.emit(
                    "install-progress",
                    InstallProgress {
                        app_id: app_id.clone(),
                        app_name: app_name.clone(),
                        status: InstallStatus::Failed,
                        message: format!("Failed to execute winget: {}", e),
                    },
                );
            }
        }
    }

    Ok(())
}
