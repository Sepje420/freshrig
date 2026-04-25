//! Shared helpers for macOS command implementations. Mirrors the public
//! surface of `commands::linux::util` where it makes sense (run_cmd, which,
//! home_dir, is_root) and adds macOS-flavoured pieces: an osascript-based
//! elevation wrapper and a Homebrew binary-path probe.

#![allow(dead_code)]

use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Standard error string used by stub commands during the platform-layer
/// transition. Kept around for any leftover callers; real implementations
/// no longer reference it.
pub const STUB_ERR: &str = "macOS support coming soon";

/// Run a command and return stdout as a String. Non-zero exit → Err with
/// the program name + stderr so the caller can surface a useful message.
/// Output is *not* trimmed — call sites trim as needed (matches Linux).
pub fn run_cmd(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to spawn {}: {}", program, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "{} exited with status {}: {}",
            program,
            output.status,
            stderr.trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run a command, returning Ok on success and the stderr-enriched error on failure.
pub fn run_cmd_ok(program: &str, args: &[&str]) -> Result<(), String> {
    run_cmd(program, args).map(|_| ())
}

/// Run a command but don't fail on a non-zero exit status — return stdout either way.
/// Useful for probes like `csrutil status` whose exit code we don't care about.
pub fn run_cmd_lossy(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

/// Return true if the requested binary is on PATH.
pub fn which(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Reads the current user's home directory from $HOME.
pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// True if euid == 0 (i.e. running via sudo).
pub fn is_root() -> bool {
    nix::unistd::geteuid().is_root()
}

/// Run a shell command via `osascript` with administrator privileges.
/// Triggers a GUI authentication prompt the first time per session. Used
/// for things like flushing DNS, modifying root-owned plists, and toggling
/// the firewall. Output is the trimmed stdout of the inner shell command.
pub fn run_elevated(shell_cmd: &str) -> Result<String, String> {
    // Escape backslashes and double-quotes so the resulting AppleScript
    // string literal is well-formed.
    let escaped = shell_cmd.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!(
        "do shell script \"{}\" with administrator privileges",
        escaped
    );
    run_cmd("osascript", &["-e", &script])
}

/// Detect the Homebrew binary path. Apple Silicon installs default to
/// `/opt/homebrew/bin/brew`; Intel installs default to `/usr/local/bin/brew`.
/// Returns `None` if neither is present.
pub fn brew_path() -> Option<&'static str> {
    if std::path::Path::new("/opt/homebrew/bin/brew").exists() {
        Some("/opt/homebrew/bin/brew")
    } else if std::path::Path::new("/usr/local/bin/brew").exists() {
        Some("/usr/local/bin/brew")
    } else {
        None
    }
}
