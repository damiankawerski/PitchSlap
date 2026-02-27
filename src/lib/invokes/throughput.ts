import { invoke } from '@tauri-apps/api/core';

export async function startThroughputInvoke() {
  try {
    await invoke('throughput');
  } catch (error) {
    console.error('Error invoking throughput:', error);
  }
}

export async function stopThroughputInvoke() {
  try {
    await invoke('stop_throughput');
  } catch (error) {
    console.error('Error invoking stop_throughput:', error);
  }
}
