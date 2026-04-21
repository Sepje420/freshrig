export const CHANGELOG: Record<string, string> = {
  "0.5.1": `### What's New in v0.5.1

- **Build fingerprint fix** — Fingerprint now baked at compile time for accurate build tracing
- **Dev experience** — UAC elevation only triggers on release builds, not during development
- **Repo consistency** — Standardized GitHub URL casing across all documentation
`,
  "0.5.0": `### What's New in v0.5.0

- **Always elevated** — FreshRig now runs as administrator automatically, no more restart prompts
- **Silent driver install** — Install GPU and hardware driver tools directly from the Drivers page via winget
- **Build traceability** — Each build carries a unique fingerprint for support
- **Stronger license validation** — Improved Pro key format validation
- **Copyright headers** — Source files now carry proper attribution
`,
  "0.4.0": `### What's New in v0.4.0

- **Stability** — WMI hardware detection now has timeouts and graceful fallbacks; never hangs on broken systems
- **Smarter winget** — JSON output mode with automatic fallback to table parsing for older Windows versions
- **Retry failed installs** — One-click retry for apps that failed during batch install
- **Expert tier** — "Risky" debloat tier renamed to "Expert" for clarity
- **Windows version awareness** — Debloat tweaks now show compatibility badges (Win11-only tweaks disabled on Win10)
- **Download estimates** — See total estimated download size before starting batch install
- **Debloat summary** — In-app results banner after applying optimizations
- **Disk space check** — Warning before installing if drive space is low
- **Network check** — Offline indicator with retry on the Apps page
- **Accessibility** — Focus trap in command palette, aria-live install progress, health score screen reader support
- **Landing page** — Comparison table vs competitors, social proof badges, trust signals
- **Security** — SBOM generation in CI, crash log scrubbing, updated security policy
- **UX polish** — Category select/deselect all, better onboarding skip, UAC warning, confetti timing, tier tooltips
`,
  "0.3.0": `### What's New in v0.3.0

- **Custom app entries** — Add your own installers with download URL, silent install switches, and SHA256 hash verification
- **Portable mode** — Run FreshRig from a USB drive with a .portable marker file — all data stays next to the executable
- **Pro tier foundation** — Pro badges and license key activation (cosmetic for now, all features remain free)
- **Windows debloating** — 23 tweaks across Safe, Moderate, and Expert tiers with mandatory restore points
- **Onboarding wizard** — First-run setup with hardware detection and preset selection
- **Command palette** — Ctrl+K spotlight search for quick navigation and actions
- **Keyboard shortcuts** — Ctrl+1-5 page navigation, Ctrl+, for settings, Ctrl+Shift+/ for shortcut reference
- **Skeleton loading** — Smooth loading states across all pages
`,
  "0.2.0": `### What's New in v0.2.0

- **Search any app** — Search the entire winget repository, not just our curated catalog
- **Installed app detection** — See which apps are already on your system
- **Preset profiles** — One-click setup for Gamers, Developers, Privacy enthusiasts, and more
- **Auto-updates** — FreshRig now checks for and installs updates automatically
- **60+ apps** — Expanded catalog with developer tools, privacy apps, and creative software
`,
  "0.1.0": `### FreshRig v0.1.0 — First Release

- Hardware detection dashboard
- Driver recommendations
- App batch install via winget
- Shareable profiles
- Settings and system tray
`,
};
