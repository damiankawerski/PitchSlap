import { invoke } from '@tauri-apps/api/core';


export async function enableModulationInvoke() {
  try {
    await invoke('enable_modulation');
  } catch (error) {
    console.error('Error invoking enable_modulation:', error);
  }
}

export async function disableModulationInvoke() {
  try {
    await invoke('disable_modulation');
  } catch (error) {
    console.error('Error invoking disable_modulation:', error);
  }
}

export async function isModulationEnabled() {
  try {
    const isEnabled: boolean = await invoke('is_modulation_active');
    return isEnabled;
  } catch (error) {
    console.error('Error invoking is_modulation_active:', error);
    return false;
  }
}


export async function appendEffect(effectName: string) {
  try {
    await invoke('append_effect', { effectName });
  } catch (error) {
    console.error('Error invoking append_effect:', error);
  }
}

export async function removeEffect(effectName: string) {
  try {
    await invoke('remove_effect', { effectName });
  } catch (error) {
    console.error('Error invoking remove_effect:', error);
  }
}

export async function set_effect_parameters(effectName: string, parameterName: string, value: number) {
  try {
    await invoke('set_effect_parameter', { effectName, parameterName, value });
  } catch (error) {
    console.error('Error invoking set_effect_parameter:', error);
  }
}

export async function set_auto_tune_scale(scale: string) {
  try {
    await invoke('set_auto_tune_scale', { scale });
  } catch (error) {
    console.error('Error invoking set_auto_tune_scale:', error);
  }
}

export async function get_auto_tune_scale() {
  try {
    const scale: string | null = await invoke('get_auto_tune_scale');
    return scale;
  } catch (error) {
    console.error('Error invoking get_auto_tune_scale:', error);
    return null;
  }
}

export async function get_active_effects() {
  try {
    const activeEffects: string[] = await invoke('get_active_effects');
    return activeEffects;
  } catch (error) {
    console.error('Error invoking get_active_effects:', error);
    return [];
  }
}

export type EffectParameter = {
  name: string;
  value: number;
  min_value: number;
  max_value: number;
  default_value: number;
};

export async function get_parameters(effectName: string): Promise<EffectParameter[]> {
  try {
    const parameters: EffectParameter[] = await invoke('get_parameters', { effectName });
    return parameters;
  } catch (error) {
    console.error('Error invoking get_parameters:', error);
    return [];
  }
}