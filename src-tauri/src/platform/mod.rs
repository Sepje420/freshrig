// Platform abstraction scaffold. Types and per-OS functions here are
// consumed incrementally as commands migrate off direct WMI/winreg calls —
// silence unused-code lints until the wiring lands.
#![allow(dead_code, unused_imports)]

pub mod types;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

// Re-export the active platform module as `current`.
#[cfg(target_os = "linux")]
pub use self::linux as current;
#[cfg(target_os = "macos")]
pub use self::macos as current;
#[cfg(target_os = "windows")]
pub use self::windows as current;
