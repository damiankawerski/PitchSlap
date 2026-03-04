import { listen } from '@tauri-apps/api/event';
import { AudioFrame } from './audio-frame';

export function listenVisualizer(isInitialized: boolean, drawSpectrum: (data: AudioFrame) => void) {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<AudioFrame>('audio-spectrum', (event) => {
        console.log(event)
        drawSpectrum(event.payload);
      });
    };

    if (isInitialized) {
      setupListener();
    }

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
}