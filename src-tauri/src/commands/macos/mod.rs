//! macOS command stubs. Mirrors `commands::linux::*` so `generate_handler!`
//! can register the same command names on macOS, but every body returns
//! `Err("macOS support coming soon".into())`. Real implementations land in a
//! later release that will shell out to `system_profiler`, `sysctl`,
//! `diskutil`, `launchctl`, `networksetup`, `ioreg`, `tmutil`, etc.

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
