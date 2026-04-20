fn main() {
    let mut attrs = tauri_build::Attributes::new();
    #[cfg(windows)]
    {
        let manifest = include_str!("windows-app-manifest.xml");
        let windows = tauri_build::WindowsAttributes::new().app_manifest(manifest);
        attrs = attrs.windows_attributes(windows);
    }
    tauri_build::try_build(attrs).expect("failed to run tauri build script");
}
