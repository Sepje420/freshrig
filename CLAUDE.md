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
- **App Config:** Never hardcode "FreshRig" in UI code — always use `src/config/app.ts`. Current version: **0.4.0**.
- **Tauri IPC:** Frontend calls `invoke('command_name')`, backend uses `#[tauri::command]` in `src-tauri/src/lib.rs`.
- **Rust ↔ TS:** Rust uses snake_case, TypeScript uses camelCase — Tauri auto-converts field names.
- **Hardware data:** All hardware info comes from WMI queries via the `wmi` crate (v0.18+, `WMIConnection::new()` takes 0 args). WMI queries have 5-second timeouts to avoid hangs.
- **Winget:** ALL winget commands MUST wrap with: `cmd /C "chcp 65001 >nul && winget ..."` (encoding fix). Uses JSON output mode with automatic table-parsing fallback for older Windows versions.
- **Design tokens:** Dark theme only — tokens defined in `src/styles.css` @theme block.
- **Serialization:** All Rust models use `#[serde(default)]` on fields for forward compatibility.
- **Storage:** Settings via `tauri-plugin-store` (`settings.json`). Profiles in `%APPDATA%/com.freshrig.app/profiles/` (or portable data dir).
- **Debloat Tiers:** Safe → Moderate → Expert (type: `TweakTier = "safe" | "moderate" | "expert"`).
- **Pre-flight checks:** Disk space (`get_free_disk_space_gb`) and network connectivity (`check_network_connectivity`) are checked before batch installs.
- **Crash logs:** Panic handler scrubs usernames, MAC addresses, and serial numbers via regex before writing to `crash.log`.
- **SBOM:** CI generates CycloneDX SBOMs for both Rust and npm dependencies.

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
