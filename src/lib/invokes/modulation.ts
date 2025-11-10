import { invoke } from '@tauri-apps/api/core';

export async function getEffectsList() {
  try {
    const effects: string[] = await invoke('get_effects_list');
    return effects;
  } catch (error) {
    console.error('Error invoking get_effects_list:', error);
    return [];
  }
}

export async function setEffectInvoke(effectName: string) {
  try {
    await invoke('set_effect', { effectName });
  } catch (error) {
    console.error('Error invoking set_effect:', error);
  }
}

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

export async function getCurrentEffect() {
  try {
    const effectName: string = await invoke('get_current_effect_name');
    return effectName;
  } catch (error) {
    console.error('Error invoking get_current_effect:', error);
    return null;
  }
}

export async function clearEffectInvoke() {
  try {
    await invoke('clear_effect');
  } catch (error) {
    console.error('Error invoking clear_effect:', error);
  }
}
