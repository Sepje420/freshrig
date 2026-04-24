use super::types::SystemInfo;
use std::fs;

pub fn get_system_info() -> SystemInfo {
    let os = os_info::get();

    let (distro_id, distro_family) = read_os_release();

    let de = std::env::var("XDG_CURRENT_DESKTOP").ok();

    let hostname = fs::read_to_string("/etc/hostname")
        .unwrap_or_default()
        .trim()
        .to_string();

    let uptime = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|v| v.parse::<f64>().ok())
        })
        .unwrap_or(0.0) as u64;

    let arch = std::env::consts::ARCH.to_string();

    SystemInfo {
        os_name: format!("{}", os.os_type()),
        os_version: os.version().to_string(),
        os_build: os.edition().unwrap_or("").to_string(),
        hostname,
        architecture: arch,
        uptime_seconds: uptime,
        desktop_environment: de,
        distro_id: Some(distro_id),
        distro_family: Some(distro_family),
    }
}

fn read_os_release() -> (String, String) {
    let content = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let mut id = String::new();
    let mut id_like = String::new();
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            id = val.trim_matches('"').to_string();
        }
        if let Some(val) = line.strip_prefix("ID_LIKE=") {
            id_like = val.trim_matches('"').to_string();
        }
    }
    let family = if !id_like.is_empty() {
        id_like.split_whitespace().next().unwrap_or(&id).to_string()
    } else {
        match id.as_str() {
            "ubuntu" | "linuxmint" | "pop" | "elementary" | "zorin" => "debian".into(),
            "fedora" | "nobara" | "centos" | "rhel" | "rocky" | "alma" => "rhel".into(),
            "arch" | "endeavouros" | "cachyos" | "manjaro" | "garuda" => "arch".into(),
            "opensuse-tumbleweed" | "opensuse-leap" => "suse".into(),
            _ => id.clone(),
        }
    };
    (id, family)
}

pub fn get_distro_family() -> Option<String> {
    let (_, family) = read_os_release();
    Some(family)
}

pub fn is_admin() -> bool {
    nix::unistd::geteuid().is_root()
}
