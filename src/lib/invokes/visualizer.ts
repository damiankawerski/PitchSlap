import { invoke } from '@tauri-apps/api/core';

export async function isVisualizerInitializedInvoke() {
  try {
    const initialized: boolean = await invoke('is_initialized');
    return initialized;
  } catch (error) {
    console.error('Error invoking is_initialized:', error);
    return false;
  }
}

export async function initializeVisualizerInvoke() {
  try {
    await invoke('initialize_audio');
  } catch (error) {
    console.error('Error invoking initialize_audio:', error);
    throw error;
  }
}

export async function deinitializeVisualizerInvoke() {
  try {
    await invoke('deinitialize_audio');
  } catch (error) {
    console.error('Error invoking deinitialize_audio:', error);
    throw error;
  }
}