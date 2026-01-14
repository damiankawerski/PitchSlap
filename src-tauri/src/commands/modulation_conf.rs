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

