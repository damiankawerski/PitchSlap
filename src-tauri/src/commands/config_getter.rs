// Current settings for the audio controls module

use crate::audio::audio_controls::*;

#[tauri::command]
pub fn get_selected_input_device() -> Result<String, String> {
    Ok(
        AudioControls::get_instance()
            .lock()
            .map_err(|e| e.to_string())?
            .get_input_device()
    )
}

#[tauri::command]
pub fn get_selected_output_device() -> Result<String, String> {
    Ok(
        AudioControls::get_instance()
            .lock()
            .map_err(|e| e.to_string())?
            .get_output_device()
    )
}

#[tauri::command]
pub fn get_selected_virtual_input() -> Result<String, String> {
    Ok(
        AudioControls::get_instance()
            .lock()
            .map_err(|e| e.to_string())?
            .get_virtual_input()
    )
}


#[tauri::command]
pub fn get_latency() -> Result<f32, String> {
    Ok(
        AudioControls::get_instance()
            .lock()
            .map_err(|e| e.to_string())?
            .get_latency()
    )
}

#[tauri::command]
pub fn is_loopback_running() -> Result<bool, String> {
    Ok(
        AudioControls::get_instance()
            .lock()
            .map_err(|e| e.to_string())?
            .is_loopback_running()
    )
}

#[tauri::command]
pub fn is_throughput_running() -> Result<bool, String> {
    Ok(
        AudioControls::get_instance()
            .lock()
            .map_err(|e| e.to_string())?
            .is_throughput_running()
    )
}
