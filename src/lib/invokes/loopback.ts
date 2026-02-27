import { invoke } from '@tauri-apps/api/core';

export async function startLoopbackInvoke() {
  try {
    await invoke('loopback');
  } catch (error) {
    console.error('Error invoking loopback:', error);
  }
}

export async function stopLoopbackInvoke() {
  try {
    await invoke('stop_loopback');
  } catch (error) {
    console.error('Error invoking stop_loopback:', error);
  }
}