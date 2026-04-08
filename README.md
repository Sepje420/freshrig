<div align="center">
  <h1>🖥️ FreshRig</h1>
  <p><strong>The first tool you run after a fresh Windows install.</strong></p>
  <p>Hardware detection • Driver recommendations • App batch install • Shareable profiles</p>

  <p>
    <a href="https://github.com/sepje420/freshrig/releases/latest"><img src="https://img.shields.io/github/v/release/sepje420/freshrig?style=flat-square&color=00d4aa" alt="Latest Release"></a>
    <a href="https://github.com/sepje420/freshrig/releases"><img src="https://img.shields.io/github/downloads/sepje420/freshrig/total?style=flat-square&color=00d4aa" alt="Downloads"></a>
    <a href="https://github.com/sepje420/freshrig/stargazers"><img src="https://img.shields.io/github/stars/sepje420/freshrig?style=flat-square&color=00d4aa" alt="Stars"></a>
    <a href="https://github.com/sepje420/freshrig/blob/main/LICENSE"><img src="https://img.shields.io/github/license/sepje420/freshrig?style=flat-square" alt="License"></a>
  </p>

  <p>
    <a href="https://github.com/sepje420/freshrig/releases/latest/download/FreshRig_0.1.0_x64-setup.exe">
      <img src="https://img.shields.io/badge/Download%20for%20Windows-00d4aa?style=for-the-badge&logo=windows&logoColor=white" alt="Download">
    </a>
  </p>
</div>

---

## What is FreshRig?

FreshRig is a **free, open-source Windows desktop app** that combines hardware detection, driver recommendations, and app batch installation into a single tool. No more juggling between Ninite, Snappy Driver Installer, and manual downloads after a fresh Windows install.

**No existing tool does all three.** FreshRig does.

## ✨ Features

- 🔍 **Hardware Dashboard** — Auto-detects your CPU, GPU, motherboard, storage, network, and audio via WMI
- 🎯 **Driver Recommendations** — Identifies your hardware vendors (NVIDIA, AMD, Intel, Realtek) and links directly to official driver downloads
- 📦 **App Batch Install** — 35+ popular apps organized by category, installed silently via winget with real-time progress
- 💾 **Shareable Profiles** — Save your app selection as a `.freshrig.json` profile, share via code or file, import on any PC
- 🏥 **Health Score** — Instant readiness assessment of your system with driver issue detection
- 🎨 **Dark Mode UI** — Premium dark theme with customizable accent colors, built with Tailwind CSS
- 🎉 **Celebration Mode** — Confetti and time-saved counter when your setup completes

## 📸 Screenshots

<!-- Add screenshots after taking them -->
*Screenshots coming soon — run `npm run tauri dev` to see the app in action.*

## 🚀 Quick Start

### Download
Get the latest installer from [GitHub Releases](https://github.com/sepje420/freshrig/releases/latest).

> ⚠️ **Windows SmartScreen may show a warning** since the app is new and unsigned. Click "More info" → "Run anyway". This is normal for new open-source apps.

### Build from source
```bash
# Prerequisites: Node.js, Rust, Microsoft C++ Build Tools
git clone https://github.com/sepje420/freshrig.git
cd freshrig
npm install
npm run tauri dev
```

## 🛠️ Tech Stack

- **Frontend**: React 19 + TypeScript + Tailwind CSS + Zustand
- **Backend**: Rust (Tauri v2) + WMI for hardware detection
- **Installer**: NSIS (via Tauri)
- **Package Manager**: winget (for app installations)

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

## 🇧🇪 Made in Belgium

Built with ❤️ by [sepje420](https://github.com/sepje420)
