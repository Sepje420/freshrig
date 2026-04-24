# FreshRig — Project Context for Claude Code

## What this project is
FreshRig is a Windows desktop app (Tauri v2 + React + TypeScript) at `C:\Users\Seppe\Desktop\PROJECTS\FreshRig` that combines hardware detection, driver recommendations, app batch installation (winget), and system optimization into one tool. Target audience: gamers, developers, PC enthusiasts.

## Tech stack
- Frontend: React 19, TypeScript, Tailwind CSS v4, Zustand, Lucide React
- Backend: Rust (Tauri v2), wmi crate for hardware detection
- Build: Vite, npm

## Project structure
- `src/` — React frontend
- `src-tauri/src/` — Rust backend (Tauri commands in `lib.rs`)
- `src/components/` — React components organized by feature
- `src/stores/` — Zustand stores
- `src/types/` — TypeScript type definitions
- `src/config/` — App constants (`app.ts`)

## Key patterns & Requirements
- **App Config:** Never hardcode "FreshRig" in UI code — always use `src/config/app.ts`. Current version: **1.1.0** (Linux support). `PRO_PURCHASE_URL`, `PRO_PRICE_LABEL`, `TRIAL_DAYS` also live in `app.ts`.
- **Tauri IPC:** Frontend calls `invoke('command_name')`, backend uses `#[tauri::command]` in `src-tauri/src/lib.rs`.
- **Rust ↔ TS:** Rust uses snake_case, TypeScript uses camelCase — Tauri auto-converts field names.
- **Hardware data:** All hardware info comes from WMI queries via the `wmi` crate (v0.18+, `WMIConnection::new()` takes 0 args). WMI queries have 5-second timeouts to avoid hangs.
- **Winget:** ALL winget commands MUST wrap with: `cmd /C "chcp 65001 >nul && winget ..."` (encoding fix). Uses JSON output mode with automatic table-parsing fallback for older Windows versions.
- **Process spawning:** Never call `Command::new("cmd")` or `Command::new("powershell")` directly. Always use `crate::util::silent_cmd()`, which sets `CREATE_NO_WINDOW` (0x08000000) on Windows so background processes don't flash console windows. The helper lives at `src-tauri/src/util.rs` and is a no-op on non-Windows.
- **Design tokens:** Dark theme only — tokens defined in `src/styles.css` @theme block.
- **Serialization:** All Rust models use `#[serde(default)]` on fields for forward compatibility.
- **Storage:** Settings via `tauri-plugin-store` (`settings.json`). Profiles in `%APPDATA%/com.freshrig.app/profiles/` (or portable data dir).
- **Debloat Tiers:** Safe → Moderate → Expert (type: `TweakTier = "safe" | "moderate" | "expert"`).
- **Pre-flight checks:** Disk space (`get_free_disk_space_gb`) and network connectivity (`check_network_connectivity`) are checked before batch installs.
- **Always elevated:** App embeds a Windows manifest (`src-tauri/windows-app-manifest.xml`) with `requireAdministrator` via `tauri_build::WindowsAttributes::app_manifest()` in `build.rs`. The manifest is embedded ONLY for release builds (`PROFILE=release`), so `npm run tauri dev` does NOT trigger UAC — only `npm run tauri build` output does. The Common-Controls v6 dependency in the manifest is mandatory — without it Tauri's dialog APIs crash. Do NOT use `embed-resource` — it causes CVT1100 duplicate-resource linker errors with Tauri v2.
- **Driver installs:** `DriverInstallAction` is a tagged enum (`Winget(String) | DirectDownload(String)`). NVIDIA and AMD GPUs route to `DirectDownload` because the NVIDIA App has no winget package and GeForce Experience is deprecated; Intel GPU/network devices use the `Intel.DriverSupportAssistant` winget id. The frontend shows a "Open download page" fallback button when a winget install fails with a hash-mismatch error.
- **Theming:** 6 accent presets via `data-accent` attribute on `<html>`. CSS vars in `src/styles.css`. Store in `settingsStore`. FOUC prevented by inline script in `index.html` that reads `localStorage["freshrig-settings"]` and sets the attribute before React mounts.
- **Animations:** `MotionConfig` at app root with spring `{ stiffness: 380, damping: 30, mass: 0.8 }`. Page transitions use `AnimatePresence mode="wait"`. Import from `"framer-motion"` (package name, even though the library was renamed to `"motion"`).
- **Startup Manager:** `StartupApproved` binary format: `byte[0]` `0x02`=enabled (`0x06` also enabled), `0x03`=disabled; `bytes[4..12]`=FILETIME of the toggle. Protected items: `SecurityHealth`, `Windows Defender`, `explorer` (case-insensitive substring match in `is_protected`).
- **Pro License Testing:** Test keys must match format `FR-XXXXX-XXXXX` where X is uppercase A-Z or 0-9. Example test key: `FR-TEST1-KEY01`. The old lowercase "FR-xxxxx" format no longer validates.
- **Pro Features (v1.0.0):** Six Pro features gated by `ProFeatureGate` (`src/components/ui/ProFeatureGate.tsx`, modes: `overlay | blur | badge | hide`). Non-Pro users see an upsell linking to `PRO_PURCHASE_URL` with `$39 one-time` and a "Start 14-day free trial" button. The Pro commands registered in `lib.rs`:
  - **Disk Cleanup** — `commands::cleanup::scan_cleanup`, `commands::cleanup::run_cleanup`
  - **Privacy Dashboard** — `commands::privacy::get_privacy_settings`, `get_app_permissions`, `apply_privacy_setting`, `revoke_app_permission`
  - **Network Tools** — `commands::network::network_reset_dns`, `network_reset_full`, `set_dns_servers`, `get_network_interfaces`, `get_wifi_passwords`
  - **Services Manager** — `commands::services::get_services`, `set_service_start_type`, `get_service_presets`, `apply_service_preset`
  - **Context Menu Editor** — `commands::context_menu::get_classic_menu_status`, `toggle_classic_menu`, `get_shell_extensions`, `toggle_shell_extension`
  - **System Health Report** — `commands::report::generate_health_report`
- **LemonSqueezy Licensing:** `commands::license::get_machine_fingerprint` (SHA-256 of MachineGuid + CPU ID + SMBIOS UUID), `activate_license` → `POST https://api.lemonsqueezy.com/v1/licenses/activate` (form-urlencoded), `validate_license` → `POST /v1/licenses/validate`. Frontend runs 6-hour revalidation tick from `App.tsx` with 14-day grace on network failure. `EXPECTED_STORE_ID` / `EXPECTED_PRODUCT_ID` in `src-tauri/src/commands/license.rs` must be filled before first real sale — when `0`, store/product match is skipped (dev-friendly).
- **Trial mode:** `useLicenseStore.startTrial()` sets `trialStartedAt` and grants Pro access locally for `TRIAL_DAYS` (14 days). `isPro()` returns true if tier is "pro" OR trial is still within the window. Trial can only be started once per install; after expiry, user must purchase.
- **SMART data:** Drive health comes from `Get-PhysicalDisk` + `Get-StorageReliabilityCounter` in `ROOT\Microsoft\Windows\Storage` WMI namespace via PowerShell wrapped with `silent_cmd`. Fields are `Option<T>` because older SATA and USB-attached drives may not populate the counter. Derivation: `Wear >= 90` or predict-failure → "Fail"; `Wear >= 70` or temp >= 60°C → "Warning"; else "OK".
- **Privacy ConsentStore path encoding:** Registry path `HKCU\Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\<capability>\NonPackaged\<path>` uses `#` to encode `\` in the Win32 path (e.g. `C:#Program Files#App#app.exe`). The `privacy::get_app_permissions` helper decodes `#` → `\` when parsing app paths.
- **FILETIME epoch delta:** Windows FILETIME is 100-ns intervals since 1601-01-01 UTC. Unix epoch delta = `11_644_473_600` seconds. Used in `startup.rs` when parsing `StartupApproved` `bytes[4..12]`.
- **Build fingerprint:** `BUILD_FINGERPRINT` in `src/config/app.ts` uses a `__BUILD_TIMESTAMP__` constant injected by Vite (`define` in `vite.config.ts`) — it's baked in at build time, so all users on the same build report the same fingerprint. Declared in `src/vite-env.d.ts`.
- **Crash logs:** Panic handler scrubs usernames, MAC addresses, and serial numbers via regex before writing to `crash.log`.
- **SBOM:** CI generates CycloneDX SBOMs for both Rust and npm dependencies.

## Known upstream issues
- **Suppressed cargo audit findings:** Two transitive Tauri deps surface RustSec advisories that we cannot patch at our layer. `glib 0.18.5` (RUSTSEC-2024-0429 / GHSA-wrw7-89jp-8q8g) is Linux-only via gtk-rs 0.18 and excluded from Windows builds. `rand 0.7.3` (RUSTSEC-2026-0097 / GHSA-cq8v-f236-94qc) is build-time HTML codegen via `kuchikiki`. Both ignored in `src-tauri/.cargo/audit.toml`. Re-evaluate when Tauri bumps to gtk-rs 0.20+ or replaces kuchikiki.

## Linux support
- Platform abstraction: `src-tauri/src/platform/` with `mod.rs`, `types.rs`, `windows.rs`, `linux.rs`.
- Linux deps (gated via `[target.'cfg(target_os = "linux")']` in `Cargo.toml`): `sysinfo`, `nix`, `procfs`, `os_info`, `cfg-if`.
- Linux hardware: `/proc/cpuinfo`, `/proc/meminfo`, `/sys/class/dmi/id`, `/sys/class/net`, `/sys/class/power_supply`, `lspci`, `lsblk -JO`, `smartctl -j`.
- Linux package managers: `apt-get`, `dnf`, `pacman`, `zypper`, `flatpak` — detected via `/etc/os-release` `ID_LIKE` (wrapped in `platform::current::get_distro_family()`).
- Linux services: `systemctl list-units --output=json` + `is-enabled` probe; same `ServiceStartType` enum as Windows.
- Linux startup: XDG autostart (`~/.config/autostart/*.desktop` + `/etc/xdg/autostart/`) and `systemctl --user` user units.
- Linux privacy: Flatpak permission audit via `flatpak info --show-permissions`, plus apport/whoopsie/popcon/firewall/auto-update toggles.
- Linux cleanup: `~/.cache/`, `/var/log/`, distro package cache, browser cache, trash, thumbnails — via `jwalk` + `trash`.
- Linux elevation: `pkexec` (polkit), never `sudo`. Matches the GUI-session expectation of Tauri apps.
- CI: matrix build on `ubuntu-22.04` + `windows-latest` (`.github/workflows/ci.yml`); release on tag push produces `.exe`, `.deb`, `.rpm`, `.AppImage` (`.github/workflows/release.yml`).
- Linux command tree: `src-tauri/src/commands/linux/` — parallel subtree mirroring the Windows command modules. `tauri::generate_handler!` entries in `lib.rs` cfg-gate a Windows twin and a Linux twin under the same command name so the frontend's `invoke()` calls are OS-agnostic.

## Commands & Workflow
- `npm run tauri dev` — start development
- `npm run tauri build` — creates production NSIS installer in `src-tauri/target/release/bundle/nsis/`
- `cargo clippy --manifest-path src-tauri/Cargo.toml` — lint Rust code
- `npx tsc --noEmit` — type-check frontend
- `npx @tauri-apps/cli icon <source>` — regenerate all icon sizes from a source image
- **Mandatory validation after EVERY phase:**
  `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings && npx tsc --noEmit`
- **Git:** Commit each completed phase separately after successful validation.

## Code conventions
- **Rust:** Use `Result<T, String>` for Tauri commands, always handle WMI errors gracefully.
- **React:** Functional components only, hooks for state, no class components.
- **TypeScript:** Strict mode, no `any` types.
- **CSS:** Tailwind v4 utility classes referencing `@theme` design tokens (e.g., `bg-bg-card`, `text-accent`).

## Design Guidelines
The UI must feel premium, dark, and modern — like a tool built by gamers for gamers. Think Discord meets HWiNFO. NOT generic light-mode corporate SaaS.

Key principles:
- **Theme:** bg-primary (#0a0a0f) near-black with blue undertone; accent (#00d4aa) electric teal — used sparingly for active states, progress, key CTAs.
- **Typography:** Segoe UI Variable (native Windows 11 font) for body; monospace for technical data (driver versions, hardware IDs).
- **Cards:** Subtle glass-morphism — semi-transparent backgrounds with blurred borders.
- **Animations:** 150ms fade-in on card mount, 200ms smooth hover transitions, skeleton pulse while loading.
- **Density:** Show real technical data (clock speeds, VRAM amounts, driver dates) — this audience wants details, not simplifications.
- **Icons:** Lucide only — no generic illustrations or stock icons.

## Release process
1. Bump version in 4 files: `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `package.json`, `src/config/app.ts`.
2. Run `cargo generate-lockfile --manifest-path src-tauri/Cargo.toml`.
3. Commit: `git commit -m "chore: release vX.Y.Z"`.
4. Tag & push: `git tag vX.Y.Z && git push origin main --tags`.
5. Pushing the tag triggers `.github/workflows/release.yml` which builds the NSIS installer and creates a draft GitHub release.

## GitHub infrastructure
- `.github/workflows/ci.yml` — CI on PR/push to main (tsc, clippy, fmt, test)
- `.github/workflows/release.yml` — Builds installer on tag push via tauri-action
- `.github/workflows/pages.yml` — Auto-deploys landing page to GitHub Pages
- `.github/ISSUE_TEMPLATE/` — Bug report and feature request forms
- `.github/dependabot.yml` — Weekly npm, cargo, and actions dependency updates
- `.github/release.yml` — Auto-categorized release notes
- `.github/FUNDING.yml` — GitHub Sponsors

## Slash commands
- `/project:release [patch|minor|major]` — Bump version, commit, and prepare tag.
- `/project:fix <issue-number>` — Read a GitHub issue and implement a fix.
- `/project:feature <description>` — Create a feature branch and implement.
