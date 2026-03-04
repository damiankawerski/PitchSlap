import { invoke } from '@tauri-apps/api/core';

export async function startRecordingInvoke() {
  try {
    await invoke('start_recording');
  } catch (error) {
    console.error('Error invoking start_recording:', error);
    throw error;
  }
}

export async function stopRecordingInvoke() {
  try {
    await invoke('stop_recording');
  } catch (error) {
    console.error('Error invoking stop_recording:', error);
    throw error;
  }
}

export async function setFileSavePathInvoke(path: string) {
  try {
    await invoke('set_file_save_path', { path });
  } catch (error) {
    console.error('Error invoking set_file_save_path:', error);
    throw error;
  }
}

export async function isRecordingInvoke() {
  try {
    const recording: boolean = await invoke('is_recording');
    return recording;
  } catch (error) {
    console.error('Error invoking is_recording:', error);
    return false;
  }
}

export async function getFileSavePathInvoke() {
  try {
    const path: string = await invoke('get_file_save_path');
    return path;
  } catch (error) {
    console.error('Error invoking get_file_save_path:', error);
    return null;
  }
}