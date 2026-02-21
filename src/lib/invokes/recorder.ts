import { invoke } from '@tauri-apps/api/core';

export async function startRecorderInvoke() {
  try {
    await invoke('start_recorder');
  } catch (error) {
    console.error('Error invoking start_recorder:', error);
    throw error;
  }
}

export async function stopRecorderInvoke() {
  try {
    await invoke('stop_recorder');
  } catch (error) {
    console.error('Error invoking stop_recorder:', error);
    throw error;
  }
}
