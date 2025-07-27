// Listing devices to front
use crate::audio::audio_controls::*;

#[tauri::command]
pub fn get_input_devices_list() -> Vec<String> {
    AudioControls::get_instance().lock().unwrap().get_input_devices_list()
}

#[tauri::command]
pub fn get_output_devices_list() -> Vec<String> {
    AudioControls::get_instance().lock().unwrap().get_output_devices_list()
}

#[tauri::command]
pub fn get_virtual_devices_list() -> Vec<String> {
    AudioControls::get_instance().lock().unwrap().get_virtual_devices_list()
}