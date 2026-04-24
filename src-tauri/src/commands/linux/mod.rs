//! Linux command implementations. Each submodule mirrors the public
//! `#[tauri::command]` surface of the corresponding Windows module under
//! `commands::*` so the frontend can call the same command names on either OS.

pub mod app_catalog;
pub mod apps;
pub mod cleanup;
pub mod drivers;
pub mod hardware;
pub mod network;
pub mod privacy;
pub mod report;
pub mod services;
pub mod startup;
pub mod util;
