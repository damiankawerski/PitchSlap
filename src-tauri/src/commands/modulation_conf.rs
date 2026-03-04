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

#[tauri::command]
pub fn set_auto_tune_scale(scale: crate::dsp::modules::effects::auto_tune::Scale) -> Result<String, String> {
    with_audio_controls(|controls| {
        controls.set_auto_tune_scale(scale)?;
        Ok("AutoTune scale set successfully".to_string())
    })
}

#[tauri::command]
pub fn get_parameters(effect_name: &str) -> Result<Vec<crate::dsp::modules::utils::EffectParameter>, String> {
    with_audio_controls(|controls| {
        let parameters = controls.get_parameters(effect_name)?;
        Ok(parameters)
    })
}

#[tauri::command]
pub fn get_active_effects() -> Result<Vec<String>, String> {
    with_audio_controls(|controls| {
        let active_effects = controls.get_active_effects();
        Ok(active_effects)
    })
}