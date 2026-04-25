//! macOS hardware detection via `sw_vers`, `scutil`, `sysctl`, and
//! `system_profiler -json`. Mirrors `commands::hardware` on Windows: the
//! same Tauri command names return the same cross-platform `models::hardware`
//! structs so the frontend is OS-agnostic.

use crate::commands::macos::util::run_cmd_lossy;
use crate::models::hardware::*;

#[tauri::command]
pub async fn get_hardware_summary() -> Result<HardwareSummary, String> {
    tokio::task::spawn_blocking(move || {
        Ok::<HardwareSummary, String>(HardwareSummary {
            system: read_system(),
            cpu: read_cpu(),
            gpus: read_gpus(),
            disks: read_disks(),
            network_adapters: read_network_adapters(),
            audio_devices: read_audio_devices(),
            motherboard: read_motherboard(),
        })
    })
    .await
    .map_err(|e| format!("hardware task failed: {}", e))?
}

#[tauri::command]
pub async fn get_driver_issues() -> Result<Vec<DriverIssue>, String> {
    // macOS does not expose Code 28-style driver errors the way Windows'
    // Device Manager does. Software Update is the system-level analog and
    // is surfaced via `drivers::get_driver_recommendations`.
    Ok(Vec::new())
}

#[tauri::command]
pub fn get_windows_build() -> u32 {
    // Sentinel: 0 means "not Windows / unknown build". Matches Linux.
    0
}

// ---- System ----

fn read_system() -> SystemInfo {
    let hostname = run_cmd_lossy("scutil", &["--get", "ComputerName"])
        .trim()
        .to_string();
    let os_version = run_cmd_lossy("sw_vers", &["-productVersion"])
        .trim()
        .to_string();
    let os_build = run_cmd_lossy("sw_vers", &["-buildVersion"])
        .trim()
        .to_string();
    let architecture = std::env::consts::ARCH.to_string();

    let mem_bytes = run_cmd_lossy("sysctl", &["-n", "hw.memsize"])
        .trim()
        .parse::<u64>()
        .unwrap_or(0);
    let total_ram_gb = (mem_bytes as f64) / 1024.0 / 1024.0 / 1024.0;
    let total_ram_gb = (total_ram_gb * 100.0).round() / 100.0;

    let uptime_seconds = parse_kern_boottime();

    SystemInfo {
        hostname,
        os_version,
        os_build,
        architecture,
        total_ram_gb,
        uptime_seconds,
    }
}

/// Parse `sysctl -n kern.boottime` → seconds since Unix epoch when the
/// machine booted, then subtract from now.
fn parse_kern_boottime() -> u64 {
    // Output looks like: { sec = 1700000000, usec = 0 } Mon Nov 14 12:00:00 2023
    let raw = run_cmd_lossy("sysctl", &["-n", "kern.boottime"]);
    let boot_secs = raw
        .split("sec = ")
        .nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0);
    if boot_secs == 0 {
        return 0;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    now.saturating_sub(boot_secs)
}

// ---- CPU ----

fn read_cpu() -> CpuInfo {
    let name = run_cmd_lossy("sysctl", &["-n", "machdep.cpu.brand_string"])
        .trim()
        .to_string();
    let manufacturer = if std::env::consts::ARCH == "aarch64" {
        "Apple".to_string()
    } else if name.contains("Intel") {
        "Intel".to_string()
    } else if name.contains("AMD") {
        "AMD".to_string()
    } else {
        String::new()
    };

    let cores = run_cmd_lossy("sysctl", &["-n", "hw.physicalcpu"])
        .trim()
        .parse::<u32>()
        .unwrap_or(0);
    let threads = run_cmd_lossy("sysctl", &["-n", "hw.logicalcpu"])
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    // hw.cpufrequency_max is unavailable on Apple Silicon — fall back to 0.
    let max_clock_mhz = run_cmd_lossy("sysctl", &["-n", "hw.cpufrequency_max"])
        .trim()
        .parse::<u64>()
        .ok()
        .map(|hz| (hz / 1_000_000) as u32)
        .unwrap_or(0);

    CpuInfo {
        name,
        manufacturer,
        cores,
        threads,
        max_clock_mhz,
    }
}

// ---- GPUs ----

fn read_gpus() -> Vec<GpuInfo> {
    let json = run_cmd_lossy("system_profiler", &["SPDisplaysDataType", "-json"]);
    let parsed: serde_json::Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let arr = match parsed.get("SPDisplaysDataType").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return Vec::new(),
    };

    arr.iter()
        .map(|gpu| {
            let name = gpu
                .get("sppci_model")
                .and_then(|v| v.as_str())
                .or_else(|| gpu.get("_name").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string();
            let manufacturer = gpu
                .get("spdisplays_vendor")
                .and_then(|v| v.as_str())
                .map(map_gpu_vendor)
                .unwrap_or_else(|| {
                    if name.to_lowercase().contains("apple") {
                        "Apple".to_string()
                    } else if name.to_lowercase().contains("intel") {
                        "Intel".to_string()
                    } else if name.to_lowercase().contains("amd")
                        || name.to_lowercase().contains("radeon")
                    {
                        "AMD".to_string()
                    } else if name.to_lowercase().contains("nvidia") {
                        "NVIDIA".to_string()
                    } else {
                        String::new()
                    }
                });
            let vram_mb = gpu
                .get("spdisplays_vram")
                .and_then(|v| v.as_str())
                .map(parse_vram_mb)
                .unwrap_or(0);

            GpuInfo {
                name,
                manufacturer,
                driver_version: String::new(),
                driver_date: String::new(),
                vram_mb,
                pnp_device_id: String::new(),
                status: 0,
            }
        })
        .collect()
}

fn map_gpu_vendor(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("apple") {
        "Apple".to_string()
    } else if lower.contains("intel") {
        "Intel".to_string()
    } else if lower.contains("amd") || lower.contains("ati") {
        "AMD".to_string()
    } else if lower.contains("nvidia") {
        "NVIDIA".to_string()
    } else {
        raw.to_string()
    }
}

/// Parse strings like "8 GB", "1024 MB", "Built-In" → u64 megabytes.
fn parse_vram_mb(s: &str) -> u64 {
    let trimmed = s.trim();
    let mut parts = trimmed.split_whitespace();
    let num = match parts.next().and_then(|n| n.parse::<f64>().ok()) {
        Some(n) => n,
        None => return 0,
    };
    let unit = parts.next().unwrap_or("MB").to_uppercase();
    match unit.as_str() {
        "GB" => (num * 1024.0) as u64,
        "MB" => num as u64,
        "TB" => (num * 1024.0 * 1024.0) as u64,
        _ => 0,
    }
}

// ---- Disks ----

fn read_disks() -> Vec<DiskInfo> {
    let mut out = Vec::new();
    out.extend(read_disks_of_type("SPNVMeDataType", "NVMe", "SSD"));
    out.extend(read_disks_of_type("SPSerialATADataType", "SATA", "Unknown"));
    out
}

/// Walk one `system_profiler -json` data type. macOS surfaces multiple
/// controller blocks each with a nested `_items` array of drives.
fn read_disks_of_type(data_type: &str, interface: &str, default_media: &str) -> Vec<DiskInfo> {
    let json = run_cmd_lossy("system_profiler", &[data_type, "-json"]);
    let parsed: serde_json::Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let arr = match parsed.get(data_type).and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return Vec::new(),
    };

    let mut out = Vec::new();
    for controller in arr {
        let drives = match controller.get("_items").and_then(|v| v.as_array()) {
            Some(d) => d,
            None => continue,
        };
        for drive in drives {
            let model = drive
                .get("device_model")
                .and_then(|v| v.as_str())
                .or_else(|| drive.get("_name").and_then(|v| v.as_str()))
                .unwrap_or("")
                .trim()
                .to_string();
            let size_bytes = drive
                .get("size_in_bytes")
                .and_then(|v| v.as_u64())
                .unwrap_or_else(|| {
                    drive
                        .get("size")
                        .and_then(|v| v.as_str())
                        .map(parse_size_bytes)
                        .unwrap_or(0)
                });
            let size_gb = (size_bytes as f64) / 1_000_000_000.0;
            let serial_number = drive
                .get("device_serial")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            out.push(DiskInfo {
                model,
                size_gb: (size_gb * 100.0).round() / 100.0,
                media_type: default_media.to_string(),
                interface_type: interface.to_string(),
                serial_number,
            });
        }
    }
    out
}

/// Parse strings like "1 TB", "500.28 GB" → bytes.
fn parse_size_bytes(s: &str) -> u64 {
    let trimmed = s.trim();
    let mut parts = trimmed.split_whitespace();
    let num = match parts.next().and_then(|n| n.parse::<f64>().ok()) {
        Some(n) => n,
        None => return 0,
    };
    let unit = parts.next().unwrap_or("B").to_uppercase();
    let mul: f64 = match unit.as_str() {
        "TB" => 1e12,
        "GB" => 1e9,
        "MB" => 1e6,
        "KB" => 1e3,
        _ => 1.0,
    };
    (num * mul) as u64
}

// ---- Network adapters ----

fn read_network_adapters() -> Vec<NetworkAdapter> {
    let raw = run_cmd_lossy("networksetup", &["-listallhardwareports"]);
    let mut out = Vec::new();

    let mut current_port: Option<String> = None;
    let mut current_device: Option<String> = None;
    let mut current_mac: Option<String> = None;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if let (Some(_port), Some(device)) = (current_port.take(), current_device.take()) {
                let mac = current_mac.take().unwrap_or_default();
                out.push(NetworkAdapter {
                    name: device.clone(),
                    manufacturer: "Apple".to_string(),
                    mac_address: mac,
                    connection_status: ifconfig_status(&device),
                    speed_mbps: 0,
                });
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("Hardware Port:") {
            current_port = Some(rest.trim().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("Device:") {
            current_device = Some(rest.trim().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("Ethernet Address:") {
            current_mac = Some(rest.trim().to_string());
        }
    }
    // Flush trailing block (no trailing blank line).
    if let (Some(_port), Some(device)) = (current_port, current_device) {
        let mac = current_mac.unwrap_or_default();
        out.push(NetworkAdapter {
            name: device.clone(),
            manufacturer: "Apple".to_string(),
            mac_address: mac,
            connection_status: ifconfig_status(&device),
            speed_mbps: 0,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn ifconfig_status(device: &str) -> String {
    let out = run_cmd_lossy("ifconfig", &[device]);
    for line in out.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("status:") {
            let s = rest.trim();
            return match s {
                "active" => "Connected".to_string(),
                "inactive" => "Disconnected".to_string(),
                other => other.to_string(),
            };
        }
    }
    String::new()
}

// ---- Audio ----

fn read_audio_devices() -> Vec<AudioDevice> {
    let json = run_cmd_lossy("system_profiler", &["SPAudioDataType", "-json"]);
    let parsed: serde_json::Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let arr = match parsed.get("SPAudioDataType").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return Vec::new(),
    };

    let mut devices = Vec::new();
    for controller in arr {
        // Each controller has nested _items for the actual devices.
        if let Some(items) = controller.get("_items").and_then(|v| v.as_array()) {
            for item in items {
                let name = item
                    .get("_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                if name.is_empty() {
                    continue;
                }
                let manufacturer = item
                    .get("coreaudio_device_manufacturer")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Apple")
                    .to_string();
                devices.push(AudioDevice {
                    name,
                    manufacturer,
                    status: "OK".to_string(),
                });
            }
        }
        // Fallback: top-level "_name".
        if let Some(name) = controller.get("_name").and_then(|v| v.as_str()) {
            if !devices.iter().any(|d| d.name == name) && !name.is_empty() {
                devices.push(AudioDevice {
                    name: name.to_string(),
                    manufacturer: "Apple".to_string(),
                    status: "OK".to_string(),
                });
            }
        }
    }
    devices
}

// ---- Motherboard ----

fn read_motherboard() -> MotherboardInfo {
    let manufacturer = "Apple".to_string();
    let product = run_cmd_lossy("sysctl", &["-n", "hw.model"])
        .trim()
        .to_string();

    // Serial number lives in SPHardwareDataType.serial_number.
    let serial_number = read_hardware_serial();
    let bios_version = run_cmd_lossy("sysctl", &["-n", "hw.bootrom_version"])
        .trim()
        .to_string();

    MotherboardInfo {
        manufacturer,
        product,
        serial_number,
        bios_version,
    }
}

fn read_hardware_serial() -> String {
    let json = run_cmd_lossy("system_profiler", &["SPHardwareDataType", "-json"]);
    let parsed: serde_json::Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    parsed
        .get("SPHardwareDataType")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("serial_number"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}
