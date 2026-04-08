mod commands;
mod data;
mod models;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Panic hook — write crash log
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("PANIC: {}", info);
        eprintln!("{}", msg);
        if let Ok(app_dir) = std::env::var("APPDATA") {
            let log_path = std::path::Path::new(&app_dir)
                .join("com.freshrig.app")
                .join("crash.log");
            let _ = std::fs::write(&log_path, &msg);
        }
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // System tray
            let show_item =
                MenuItem::with_id(app, "show", "Show FreshRig", true, None::<&str>)?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let scan_item =
                MenuItem::with_id(app, "scan", "Quick Scan", true, None::<&str>)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit_item =
                MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_item, &sep1, &scan_item, &sep2, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app_handle: &AppHandle, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app_handle.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app_handle = tray.app_handle();
                        if let Some(window) = app_handle.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::hardware::get_hardware_summary,
            commands::hardware::get_driver_issues,
            commands::drivers::get_driver_recommendations,
            commands::apps::get_app_catalog,
            commands::apps::check_winget_available,
            commands::apps::install_apps,
            commands::profiles::save_profile,
            commands::profiles::load_profile,
            commands::profiles::list_profiles,
            commands::profiles::delete_profile,
            commands::profiles::export_profile_to_file,
            commands::profiles::import_profile_from_file,
            commands::profiles::export_profile_as_text,
            commands::profiles::compress_profile,
            commands::profiles::decompress_profile,
            commands::profiles::get_current_hardware_snapshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
