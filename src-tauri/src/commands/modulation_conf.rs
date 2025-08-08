use crate::audio::audio_controls::*;

fn with_audio_controls<F, R>(operation: F) -> Result<R, String>
where
    F: FnOnce(&mut AudioControls) -> anyhow::Result<R>,
{
    match AudioControls::get_instance().lock() {
        Ok(mut audio_controls) => {
            operation(&mut audio_controls)
                .map_err(|e| format!("Audio operation failed: {}", e))
        }
        Err(e) => Err(format!("Failed to acquire audio controls lock: {}", e))
    }
}

#[tauri::command]
pub fn get_effects_list() -> Vec<String> {
    AudioControls::get_instance().lock().unwrap().get_effects_list()
}

#[tauri::command]
pub fn set_effect(effect_name: &str) -> Result<(), String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| format!("Failed to acquire audio controls lock: {}", e))?
        .set_effect(effect_name)
        .map_err(|e| format!("Failed to set effect: {}", e))
}

#[tauri::command]
pub fn enable_modulation() -> Result<String, String> {
    with_audio_controls(|controls| {
        println!("Enabling modulation");
        controls.enable_modulation()?;
        Ok("Modulation enabled successfully".to_string())
    })
}

#[tauri::command]
pub fn disable_modulation() -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.disable_modulation()?;
        Ok("Modulation disabled successfully".to_string())
    })
}

#[tauri::command]
pub fn is_modulation_active() -> Result<bool, String> {
    Ok(AudioControls::get_instance()
        .lock()
        .map_err(|e| format!("Failed to acquire audio controls lock: {}", e))?
        .is_modulation_active())
}

#[tauri::command]
pub fn get_current_effect_name() -> Result<Option<String>, String> {
    AudioControls::get_instance()
        .lock()
        .map_err(|e| format!("Failed to acquire audio controls lock: {}", e))?
        .get_current_effect_name()
        .map(Some)
        .ok_or_else(|| "Failed to get current effect name".to_string())
}

#[tauri::command]
pub fn clear_effect() -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.clear_effect()?;
        Ok("Effect cleared successfully".to_string())
    })
}