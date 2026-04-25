//! macOS helper scaffolding. Real shell-out helpers (mirroring
//! `commands::linux::util`) land alongside the first real macOS command
//! implementation. Kept as an empty module so the subtree's shape matches
//! Linux for code-review symmetry.

#![allow(dead_code)]

/// Standard error string returned by every stubbed macOS Tauri command.
/// Centralized so a future find-replace can swap the wording in one place.
pub const STUB_ERR: &str = "macOS support coming soon";
