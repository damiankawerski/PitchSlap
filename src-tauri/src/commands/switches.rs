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

// Refactored versions using helper function
#[tauri::command]
pub fn loopback() -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.start_audio_engine_loopback()?;
        Ok("Loopback started successfully".to_string())
    })
}

#[tauri::command]
pub fn stop_loopback() -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.stop_audio_engine_loopback()?;
        Ok("Loopback stopped successfully".to_string())
    })
}

#[tauri::command]
pub fn throughput() -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.start_audio_engine_throughput()?;
        Ok("Throughput started successfully".to_string())
    })
}

#[tauri::command]
pub fn stop_throughput() -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.stop_audio_engine_throughput()?;
        Ok("Throughput stopped successfully".to_string())
    })
}