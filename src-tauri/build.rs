fn main() {
    #[allow(unused_mut)]
    let mut attrs = tauri_build::Attributes::new();
    #[cfg(windows)]
    {
        let is_release = std::env::var("PROFILE")
            .map(|p| p == "release")
            .unwrap_or(false);
        if is_release {
            let manifest = include_str!("windows-app-manifest.xml");
            let windows = tauri_build::WindowsAttributes::new().app_manifest(manifest);
            attrs = attrs.windows_attributes(windows);
        }
    }
    tauri_build::try_build(attrs).expect("failed to run tauri build script");
}
