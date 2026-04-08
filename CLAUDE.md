# FreshRig — Project Context for Claude Code

## What this project is
FreshRig is a Windows desktop app (Tauri v2 + React + TypeScript) that combines driver detection and app installation into one tool. Target audience: gamers, developers, PC enthusiasts.

## Tech stack
- Frontend: React 19, TypeScript, Tailwind CSS v4, Zustand, Lucide React
- Backend: Rust (Tauri v2), wmi crate for hardware detection
- Build: Vite, npm

## Project structure
- `src/` — React frontend
- `src-tauri/src/` — Rust backend
- `src/components/` — React components organized by feature
- `src/stores/` — Zustand stores
- `src/types/` — TypeScript type definitions
- `src/config/` — App constants (name, version)

## Key patterns
- App name is in `src/config/app.ts` — never hardcode "FreshRig" in UI code
- Tauri IPC: frontend calls `invoke('command_name')`, backend has `#[tauri::command]` functions
- Rust uses snake_case, TypeScript uses camelCase — Tauri auto-converts
- All hardware data comes from WMI queries via the `wmi` crate
- Dark theme only, design tokens in `src/styles.css` @theme block

## Commands
- `npm run tauri dev` — start development
- `npm run tauri build` — creates production NSIS installer in `src-tauri/target/release/bundle/nsis/`
- `cargo clippy --manifest-path src-tauri/Cargo.toml` — lint Rust code
- `npx tsc --noEmit` — type-check frontend
- `npx @tauri-apps/cli icon <source>` — regenerates all icon sizes from a source image

## Code conventions
- Rust: use `Result<T, String>` for Tauri commands, always handle WMI errors gracefully
- React: functional components only, hooks for state, no class components
- TypeScript: strict mode, no `any` types
- CSS: use Tailwind utility classes referencing design tokens (e.g., `bg-bg-card`, `text-accent`)

## Design Guidelines
The UI must feel premium, dark, and modern — like a tool built by gamers for gamers. Think Discord meets HWiNFO. NOT generic light-mode corporate SaaS.

Key principles:
- Dark mode only — the bg-primary (#0a0a0f) is a near-black with a slight blue undertone
- Accent color: electric teal/cyan (#00d4aa) — used sparingly for active states, progress indicators, and key CTAs
- Typography: Use Segoe UI Variable (native Windows 11 font) for body, monospace for technical data (driver versions, hardware IDs)
- Cards: Subtle glass-morphism effect — semi-transparent backgrounds with blurred borders
- Animations: Subtle fade-in on card mount (150ms), smooth hover transitions (200ms), skeleton pulse while loading
- Data density: Show real technical data (clock speeds, VRAM amounts, driver dates) — this audience wants details, not simplifications
- No generic illustrations or stock icons — use Lucide icons consistently

## Release process
1. Bump version in 4 files: `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `package.json`, `src/config/app.ts`
2. Run `cargo generate-lockfile --manifest-path src-tauri/Cargo.toml`
3. Commit: `git commit -m "chore: release vX.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push origin main --tags`
6. Pushing the tag triggers `.github/workflows/release.yml` which builds and creates a draft GitHub release

## GitHub infrastructure
- `.github/workflows/ci.yml` — CI on PR/push to main (tsc, clippy, fmt, test)
- `.github/workflows/release.yml` — Builds installer on tag push via tauri-action
- `.github/ISSUE_TEMPLATE/` — Bug report and feature request forms
- `.github/dependabot.yml` — Weekly npm, cargo, and actions dependency updates
- `.github/release.yml` — Auto-categorized release notes
- `.github/FUNDING.yml` — GitHub Sponsors

## Slash commands
- `/project:release [patch|minor|major]` — Bump version, commit, and prepare tag
- `/project:fix <issue-number>` — Read a GitHub issue and implement a fix
- `/project:feature <description>` — Create a feature branch and implement
