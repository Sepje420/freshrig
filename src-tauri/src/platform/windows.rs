//! Windows platform stubs. Existing Windows command modules call WMI/registry
//! directly — this module exists mainly so Linux can mirror the same surface.

use super::types::SystemInfo;

pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os_name: "Windows".into(),
        architecture: std::env::consts::ARCH.to_string(),
        ..Default::default()
    }
}

pub fn get_distro_family() -> Option<String> {
    None
}

pub fn is_admin() -> bool {
    // FreshRig embeds a requireAdministrator manifest in release builds, so
    // when installed it's always elevated. Dev builds may not be.
    true
}
