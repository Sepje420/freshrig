// Cross-platform modules — always compiled on both Windows and Linux.
pub mod presets;

// Windows-only modules — bodies rely on WMI, registry, winget, shell
// extensions, or other Win32-specific APIs. Linux implementations land in
// a follow-up commit; the frontend hides the corresponding pages behind
// `usePlatform().isWindows`.
#[cfg(target_os = "windows")]
pub mod apps;
#[cfg(target_os = "windows")]
pub mod cleanup;
#[cfg(target_os = "windows")]
pub mod context_menu;
#[cfg(target_os = "windows")]
pub mod custom_apps;
#[cfg(target_os = "windows")]
pub mod debloat;
#[cfg(target_os = "windows")]
pub mod drivers;
#[cfg(target_os = "windows")]
pub mod hardware;
#[cfg(target_os = "windows")]
pub mod installed_apps;
#[cfg(target_os = "windows")]
pub mod license;
#[cfg(target_os = "windows")]
pub mod network;
#[cfg(target_os = "windows")]
pub mod privacy;
#[cfg(target_os = "windows")]
pub mod profiles;
#[cfg(target_os = "windows")]
pub mod report;
#[cfg(target_os = "windows")]
pub mod services;
#[cfg(target_os = "windows")]
pub mod startup;
#[cfg(target_os = "windows")]
pub mod winget_search;
