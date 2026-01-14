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
