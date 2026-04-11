use crate::data::preset_profiles::{get_preset_profiles, PresetProfile};

#[tauri::command]
pub fn get_presets() -> Vec<PresetProfile> {
    get_preset_profiles()
}
