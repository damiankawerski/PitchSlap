import { invoke } from '@tauri-apps/api/core';

export async function setInputDeviceInvoke(deviceName: string) {
  try {
    await invoke('set_input_device', { deviceName });
  } catch (error) {
    console.error('Error invoking set_input_device:', error);
  }
}

export async function setOutputDeviceInvoke(deviceName: string) {
  try {
    await invoke('set_output_device', { deviceName });
  } catch (error) {
    console.error('Error invoking set_output_device:', error);
  }
}

export async function setVirtualDeviceInvoke(deviceName: string) {
  try {
    await invoke('set_virtual_device', { deviceName });
  } catch (error) {
    console.error('Error invoking set_virtual_device:', error);
  }
}

export async function setLatencyInvoke(latency: number) {
  try {
    await invoke('set_latency', { latency });
  } catch (error) {
    console.error('Error invoking set_latency:', error);
  }
}
