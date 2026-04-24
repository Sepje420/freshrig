<div align="center">
  <h1>🖥️ FreshRig</h1>
  <p><strong>The first tool you run after a fresh Windows install.</strong></p>
  <p>Hardware detection • Driver recommendations • App batch install • Shareable profiles</p>

  <p>
    <a href="https://github.com/ZIPREX420/freshrig/releases/latest"><img src="https://img.shields.io/github/v/release/ZIPREX420/freshrig?style=flat-square&color=00d4aa" alt="Latest Release"></a>
    <a href="https://github.com/ZIPREX420/freshrig/releases"><img src="https://img.shields.io/github/downloads/ZIPREX420/freshrig/total?style=flat-square&color=00d4aa" alt="Downloads"></a>
    <a href="https://github.com/ZIPREX420/freshrig/stargazers"><img src="https://img.shields.io/github/stars/ZIPREX420/freshrig?style=flat-square&color=00d4aa" alt="Stars"></a>
    <a href="https://github.com/ZIPREX420/freshrig/blob/main/LICENSE"><img src="https://img.shields.io/github/license/ZIPREX420/freshrig?style=flat-square" alt="License"></a>
  </p>

  <p>
    <a href="https://github.com/ZIPREX420/freshrig/releases/latest">
      <img src="https://img.shields.io/badge/Download%20for%20Windows-00d4aa?style=for-the-badge&logo=windows&logoColor=white" alt="Download">
    </a>
  </p>
</div>

---

## What is FreshRig?

FreshRig is a **free, open-source Windows desktop app** that combines hardware detection, driver recommendations, and app batch installation into a single tool. No more juggling between Ninite, Snappy Driver Installer, and manual downloads after a fresh Windows install.

**No existing tool does all three.** FreshRig does.

## Platform Support

| Platform | Status | Package Formats |
|---|---|---|
| Windows 10/11 | ✅ Stable | .exe (NSIS installer) |
| Ubuntu / Mint / Pop!_OS | ✅ Stable | .deb, .AppImage |
| Fedora / RHEL | ✅ Stable | .rpm, .AppImage |
| Arch / EndeavourOS / Manjaro | ✅ Stable | .AppImage, AUR |
| openSUSE | ✅ Stable | .rpm, .AppImage |
| macOS | 🔜 Planned | — |

## ✨ Features

### Free (Forever)
- 🔍 **Hardware Dashboard** — Auto-detects your CPU, GPU, motherboard, storage, network, and audio via WMI
- 🎯 **Driver Finder** — Detects your hardware vendors (NVIDIA, AMD, Intel, Realtek); Intel DSA installs silently via winget, NVIDIA and AMD open their official driver tools in your browser
- 📦 **App Batch Install** — 60+ popular apps organized by category, installed silently via winget with real-time progress
- 💾 **Shareable Profiles** — Save your app selection as a `.freshrig.json` profile, share via code or file, import on any PC
- ⚙️ **Windows Optimization** — 23 tweaks across Safe, Moderate, and Expert tiers with mandatory restore points
- 🚀 **Startup Manager** — View and control Windows startup programs from registry Run keys and the Startup folder, with one-click enable/disable
- 🏥 **Health Score** — Instant readiness assessment of your system with driver issue detection
- 🎨 **6 Accent Themes** — Teal, Blue, Purple, Orange, Rose, and Green with instant switching
- ⌨️ **Command Palette** — Ctrl+K spotlight search for quick navigation and actions
- 🔄 **Auto-Updater** — Built-in update checks with passive install
- 🌙 **Premium Dark UI** — Spring-physics animations, refined cards, and polished page transitions built with Tailwind CSS
- 🎉 **Celebration Mode** — Confetti and time-saved counter when your setup completes

### Pro ($39 One-Time · No Subscription · 14-Day Free Trial · 3 Activations)
- 🧹 **Disk Cleanup** — Scan temp files, browser caches, crash dumps, shader caches with per-category risk ratings and preview before cleaning
- 🛡️ **Privacy Dashboard** — Audit camera/microphone/location permissions per app, 20+ privacy toggles with drift detection
- 🌐 **Network Tools** — One-click DNS flush, full network reset, DNS preset switching (Cloudflare, Google, Quad9), saved WiFi password viewer
- ⚡ **Services Manager** — Gaming / Privacy / Performance presets with never-disable guardrails on critical system services
- 🖱️ **Context Menu Editor** — Restore Windows 11 classic right-click menu with one toggle, view and block shell extensions
- 📄 **System Health Report** — Comprehensive PC diagnostic with hardware audit, SMART disk health, battery wear, security status, and an overall A-F grade. Export as PDF.
- 🗓️ **Scheduled Maintenance** (coming soon)
- 🏷️ **White-Label Branding** (coming soon)

## 💎 Pro vs Free

| Feature | Free | Pro |
| --- | :---: | :---: |
| Hardware Dashboard, Driver Finder, App Batch Install | ✅ | ✅ |
| Shareable Profiles, Windows Optimization, Startup Manager | ✅ | ✅ |
| 6 Accent Themes, Command Palette, Auto-Updater | ✅ | ✅ |
| Disk Cleanup | — | ⭐ |
| Privacy Dashboard | — | ⭐ |
| Network Tools | — | ⭐ |
| Services Manager | — | ⭐ |
| Context Menu Editor | — | ⭐ |
| System Health Report (PDF) | — | ⭐ |
| Scheduled Maintenance (coming soon) | — | ⭐ |
| White-Label Branding (coming soon) | — | ⭐ |

**Pro pricing:** $39 one-time · No subscription · 14-day free trial · 3 activations (covers reinstalls).

## 📸 Screenshots

See the [landing page](https://ZIPREX420.github.io/freshrig/) for screenshots and details.

## 🚀 Quick Start

### Download
Download the latest release for [Windows (.exe)](https://github.com/ZIPREX420/freshrig/releases/latest) or [Linux (.deb / .rpm / .AppImage)](https://github.com/ZIPREX420/freshrig/releases/latest).

> ⚠️ **Windows SmartScreen may show a warning** since the app is new and unsigned. Click "More info" → "Run anyway". This is normal for new open-source apps.

### Build from source
```bash
# Prerequisites: Node.js, Rust, Microsoft C++ Build Tools
git clone https://github.com/ZIPREX420/freshrig.git
cd freshrig
npm install
npm run tauri dev
```

## 🛠️ Tech Stack

- **Frontend**: React 19 + TypeScript + Tailwind CSS + Zustand
- **Backend**: Rust (Tauri v2) + WMI for hardware detection
- **Installer**: NSIS (via Tauri)
- **Package Manager**: winget (for app installations)

## 🔒 Security

Found a security issue? See [SECURITY.md](SECURITY.md).

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

## 🇧🇪 Made in Belgium

Built with ❤️ by [ZIPREX420](https://github.com/ZIPREX420)
