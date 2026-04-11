use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresetProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub app_ids: Vec<String>,
}

pub fn get_preset_profiles() -> Vec<PresetProfile> {
    vec![
        PresetProfile {
            id: "essentials".to_string(),
            name: "Essentials".to_string(),
            description:
                "The basics everyone needs — browser, archiver, media player, and power tools"
                    .to_string(),
            icon: "Zap".to_string(),
            color: "#00d4aa".to_string(),
            app_ids: vec![
                "Google.Chrome".to_string(),
                "Mozilla.Firefox".to_string(),
                "7zip.7zip".to_string(),
                "VideoLAN.VLC".to_string(),
                "Microsoft.PowerToys".to_string(),
                "voidtools.Everything".to_string(),
                "Bitwarden.Bitwarden".to_string(),
                "Microsoft.VCRedist.2015+.x64".to_string(),
            ],
        },
        PresetProfile {
            id: "gamer".to_string(),
            name: "Gamer".to_string(),
            description: "Game launchers, voice chat, GPU tools, and streaming essentials"
                .to_string(),
            icon: "Gamepad2".to_string(),
            color: "#8b5cf6".to_string(),
            app_ids: vec![
                "Valve.Steam".to_string(),
                "EpicGames.EpicGamesLauncher".to_string(),
                "GOG.Galaxy".to_string(),
                "Discord.Discord".to_string(),
                "Nvidia.GeForceExperience".to_string(),
                "REALiX.HWiNFO".to_string(),
                "TechPowerUp.GPU-Z".to_string(),
                "7zip.7zip".to_string(),
                "VideoLAN.VLC".to_string(),
                "ShareX.ShareX".to_string(),
                "OBSProject.OBSStudio".to_string(),
                "Microsoft.VCRedist.2015+.x64".to_string(),
            ],
        },
        PresetProfile {
            id: "developer".to_string(),
            name: "Developer".to_string(),
            description: "IDE, version control, runtimes, and developer utilities".to_string(),
            icon: "Code".to_string(),
            color: "#3b82f6".to_string(),
            app_ids: vec![
                "Microsoft.VisualStudioCode".to_string(),
                "Git.Git".to_string(),
                "GitHub.cli".to_string(),
                "GitHub.GitHubDesktop".to_string(),
                "OpenJS.NodeJS.LTS".to_string(),
                "Python.Python.3.12".to_string(),
                "Rustlang.Rustup".to_string(),
                "Docker.DockerDesktop".to_string(),
                "Microsoft.WindowsTerminal".to_string(),
                "Microsoft.PowerShell".to_string(),
                "JetBrains.Toolbox".to_string(),
                "Postman.Postman".to_string(),
                "Notepad++.Notepad++".to_string(),
                "DevToys-app.DevToys".to_string(),
            ],
        },
        PresetProfile {
            id: "privacy".to_string(),
            name: "Privacy-Focused".to_string(),
            description: "Privacy browsers, encrypted messaging, VPN, and password managers"
                .to_string(),
            icon: "Shield".to_string(),
            color: "#22c55e".to_string(),
            app_ids: vec![
                "Mozilla.Firefox".to_string(),
                "BraveSoftware.BraveBrowser".to_string(),
                "Mozilla.Thunderbird".to_string(),
                "Bitwarden.Bitwarden".to_string(),
                "KeePassXCTeam.KeePassXC".to_string(),
                "OpenWhisperSystems.Signal".to_string(),
                "ProtonTechnologies.ProtonVPN".to_string(),
                "IDRIX.VeraCrypt".to_string(),
            ],
        },
        PresetProfile {
            id: "creator".to_string(),
            name: "Content Creator".to_string(),
            description: "Streaming, video editing, audio, graphics, and screen capture"
                .to_string(),
            icon: "Palette".to_string(),
            color: "#f97316".to_string(),
            app_ids: vec![
                "OBSProject.OBSStudio".to_string(),
                "BlackmagicDesign.DaVinciResolve".to_string(),
                "HandBrake.HandBrake".to_string(),
                "Audacity.Audacity".to_string(),
                "GIMP.GIMP".to_string(),
                "dotPDN.PaintDotNet".to_string(),
                "Inkscape.Inkscape".to_string(),
                "BlenderFoundation.Blender".to_string(),
                "ShareX.ShareX".to_string(),
                "Discord.Discord".to_string(),
            ],
        },
        PresetProfile {
            id: "productivity".to_string(),
            name: "Productivity".to_string(),
            description: "Office suite, note-taking, cloud storage, and communication tools"
                .to_string(),
            icon: "Briefcase".to_string(),
            color: "#ec4899".to_string(),
            app_ids: vec![
                "Google.Chrome".to_string(),
                "Mozilla.Firefox".to_string(),
                "SlackTechnologies.Slack".to_string(),
                "Zoom.Zoom".to_string(),
                "Notion.Notion".to_string(),
                "Obsidian.Obsidian".to_string(),
                "TheDocumentFoundation.LibreOffice".to_string(),
                "Adobe.Acrobat.Reader.64-bit".to_string(),
                "7zip.7zip".to_string(),
                "Microsoft.PowerToys".to_string(),
                "voidtools.Everything".to_string(),
                "Google.GoogleDrive".to_string(),
            ],
        },
    ]
}
