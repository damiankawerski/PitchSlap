import { invoke } from '@tauri-apps/api/core';

export async function getSelectedInputDevice() {
  try {
    const deviceName: string = await invoke('get_selected_input_device');
    return deviceName;
  } catch (error) {
    console.error('Error invoking get_selected_input_device:', error);
    return null;
  }
}

export async function getSelectedOutputDevice() {
  try {
    const deviceName: string = await invoke('get_selected_output_device');
    return deviceName;
  } catch (error) {
    console.error('Error invoking get_selected_output_device:', error);
    return null;
  }
}

export async function getSelectedVirtualDevice() {
  try {
    const deviceName: string = await invoke('get_selected_virtual_device');
    return deviceName;
  } catch (error) {
    console.error('Error invoking get_selected_virtual_device:', error);
    return null;
  }
}

export async function getLatency() {
  try {
    const latency: number = await invoke('get_latency');
    return latency;
  } catch (error) {
    console.error('Error invoking get_latency:', error);
    return null;
  }
}

export async function getAvailableInputDevices() {
  try {
    const devices: string[] = await invoke('get_input_devices_list');
    return devices;
  } catch (error) {
    console.error('Error invoking get_available_input_devices:', error);
    return [];
  }
}

export async function getAvailableOutputDevices() {
  try {
    const devices: string[] = await invoke('get_output_devices_list');
    return devices;
  } catch (error) {
    console.error('Error invoking get_available_output_devices:', error);
    return [];
  }
}

export async function getAvailableVirtualDevices() {
  try {
    const devices: string[] = await invoke('get_virtual_devices_list');
    return devices;
  } catch (error) {
    console.error('Error invoking get_available_virtual_devices:', error);
    return [];
  }
}

export async function isLoopbackRunning() {
  try {
    const isRunning: boolean = await invoke('is_loopback_running');
    return isRunning;
  } catch (error) {
    console.error('Error invoking is_loopback_running:', error);
    return false;
  }
}

export async function isThroughputRunning() {
  try {
    const isRunning: boolean = await invoke('is_throughput_running');
    return isRunning;
  } catch (error) {
    console.error('Error invoking is_throughput_running:', error);
    return false;
  }
}
