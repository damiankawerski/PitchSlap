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
pub fn append_effect(effect_name: &str) -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.append_effect(effect_name)?;
        Ok(format!("Effect '{}' appended successfully", effect_name))
    })
}

#[tauri::command]
pub fn remove_effect(effect_name: &str) -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.remove_effect(effect_name)?;
        Ok(format!("Effect '{}' removed successfully", effect_name))
    })
}

#[tauri::command]
pub fn set_effect_parameter(effect_name: &str, parameter_name: &str, value: f32) -> Result<String, String> {
    with_audio_controls(|controls| {
        let parameter = crate::dsp::modules::utils::ParameterValue {
            name: parameter_name.to_string(),
            value,
        };
        controls.set_effect_parameter(effect_name, parameter)?;
        Ok(format!("Parameter '{}' of effect '{}' set to {}", parameter_name, effect_name, value))
    })
}