// Commands for setting audio devices

use crate::audio::audio_controls::*;

#[tauri::command]
pub fn set_input_device(device_name: String) -> Result<(), String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| e.to_string())?
        .set_input_device(&device_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_output_device(device_name: String) -> Result<(), String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| e.to_string())?
        .set_output_device(&device_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_virtual_device(device_name: String) -> Result<(), String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| e.to_string())?
        .set_virtual_input(&device_name)
        .map_err(|e| e.to_string())
}