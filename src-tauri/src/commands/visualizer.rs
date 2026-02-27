use crate::audio::audio_controls::*;
use tauri::AppHandle;

#[tauri::command]
pub fn initialize_audio(app_handle: AppHandle) -> Result<(), String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| e.to_string())?
        .set_app_handle(app_handle)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn deinitialize_audio() -> Result<(), String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| e.to_string())?
        .clear_app_handle()
        .map_err(|e| e.to_string())
}
