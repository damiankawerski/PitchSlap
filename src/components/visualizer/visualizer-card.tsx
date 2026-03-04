import { useEffect, useRef, useState } from 'react';
import { isVisualizerInitializedInvoke, initializeVisualizerInvoke, deinitializeVisualizerInvoke } from '../../lib/invokes/visualizer';
import { listenVisualizer } from '../../lib/utils/listener';

interface AudioFrame {
  rms: number;
  pitch: number;
  spectrum: number[];
  frequencies: number[];
  timestamp: number;
}

export function AudioSpectrumVisualizer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  const [fps, setFps] = useState(0);
  const [peakFrequency, setPeakFrequency] = useState(0);

  const previousMagnitudesRef = useRef<number[]>([]);
  const frameCountRef = useRef(0);
  const lastFpsUpdateRef = useRef(Date.now());

  useEffect(() => {
    const checkInitialization = async () => {
      const initialized = await isVisualizerInitializedInvoke();
      setIsInitialized(initialized);
    };

    checkInitialization();
  }, []);

  // Smoothing funkcja dla płynniejszej animacji
  const smoothSpectrum = (current: number[]): number[] => {
    const smoothingFactor = 0.7;
    const previous = previousMagnitudesRef.current;

    if (previous.length === 0) {
      previousMagnitudesRef.current = [...current];
      return current;
    }

    const smoothed = current.map((val, i) =>
      val * (1 - smoothingFactor) + (previous[i] || 0) * smoothingFactor
    );

    previousMagnitudesRef.current = smoothed;
    return smoothed;
  };

  // Rysowanie spektrum
  const drawSpectrum = (frame: AudioFrame) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    console.log(frame)

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const { spectrum: magnitudes, frequencies } = frame;
    const width = canvas.width;
    const height = canvas.height;

    // Wyczyść canvas
    ctx.fillStyle = '#0a0a0a';
    ctx.fillRect(0, 0, width, height);

    // Smooth data
    const smoothed = smoothSpectrum(magnitudes);

    // Znajdź zakres dla normalizacji
    const maxMagnitude = Math.max(...smoothed);
    const minMagnitude = Math.min(...smoothed);
    const range = maxMagnitude - minMagnitude || 1;

    // Znajdź peak frequency
    const peakIndex = smoothed.indexOf(maxMagnitude);
    setPeakFrequency(Math.round(frequencies[peakIndex]));

    // Rysuj słupki
    const barWidth = width / smoothed.length;
    const barGap = 1;

    smoothed.forEach((magnitude, i) => {
      const normalized = (magnitude - minMagnitude) / range;
      const barHeight = normalized * height * 0.9;

      // Gradient kolorów na podstawie częstotliwości i amplitudy
      const hue = (i / smoothed.length) * 280; // 0 (czerwony) do 280 (fiolet)
      const lightness = 40 + normalized * 30; // Jaśniejsze dla głośniejszych

      // Główny słupek
      ctx.fillStyle = `hsl(${hue}, 100%, ${lightness}%)`;
      ctx.fillRect(
        i * barWidth,
        height - barHeight,
        barWidth - barGap,
        barHeight
      );

      // Odbicie (efekt lustrzany na dole)
      const gradient = ctx.createLinearGradient(0, height, 0, height + barHeight * 0.3);
      gradient.addColorStop(0, `hsla(${hue}, 100%, ${lightness}%, 0.3)`);
      gradient.addColorStop(1, `hsla(${hue}, 100%, ${lightness}%, 0)`);
      ctx.fillStyle = gradient;
      ctx.fillRect(
        i * barWidth,
        height,
        barWidth - barGap,
        barHeight * 0.3
      );
    });

    // Rysuj linię częstotliwości
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
    ctx.lineWidth = 2;
    ctx.beginPath();

    smoothed.forEach((magnitude, i) => {
      const normalized = (magnitude - minMagnitude) / range;
      const x = i * barWidth + barWidth / 2;
      const y = height - (normalized * height * 0.9);

      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });

    ctx.stroke();

    // Update FPS
    frameCountRef.current++;
    const now = Date.now();
    if (now - lastFpsUpdateRef.current >= 1000) {
      setFps(frameCountRef.current);
      frameCountRef.current = 0;
      lastFpsUpdateRef.current = now;
    }
  };

  // Nasłuchuj na events
  useEffect(() => {
    const cleanup = listenVisualizer(isInitialized, (event) => {
      const frame = event as AudioFrame;
      drawSpectrum(frame);
    });
    return cleanup;
  }, [isInitialized]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 p-8">
      <div className="w-full max-w-6xl">
        {/* Header */}
        <div className="mb-8 text-center">
          <h1 className="text-4xl font-bold text-white mb-2">
            Audio Spectrum Visualizer
          </h1>
          <p className="text-gray-400">
            Real-time FFT visualization from Rust backend
          </p>
        </div>

        {/* Canvas */}
        <div className="relative bg-black rounded-lg overflow-hidden shadow-2xl mb-6">
          <canvas
            ref={canvasRef}
            width={1200}
            height={400}
            className="w-full"
          />

          {/* Stats overlay */}
          <div className="absolute top-4 right-4 bg-black bg-opacity-70 text-white px-4 py-2 rounded-lg font-mono text-sm">
            <div>FPS: {fps}</div>
            <div>Peak: {peakFrequency} Hz</div>
          </div>
        </div>

        {/* Controls */}
        <div className="flex flex-wrap gap-4 justify-center items-center">
          <button
            onClick={() => initializeVisualizerInvoke().then(() => setIsInitialized(true))}
            disabled={isInitialized}
            className={`px-6 py-3 rounded-lg font-semibold transition-all ${isInitialized
                ? 'bg-gray-600 text-gray-400 cursor-not-allowed'
                : 'bg-blue-600 hover:bg-blue-700 text-white shadow-lg hover:shadow-xl'
              }`}
          >
            Initialize Visualizer
          </button>

          <button
            onClick={() => deinitializeVisualizerInvoke().then(() => setIsInitialized(false))}
            disabled={!isInitialized}
            className={`px-6 py-3 rounded-lg font-semibold transition-all ${!isInitialized
                ? 'bg-gray-600 text-gray-400 cursor-not-allowed'
                : 'bg-orange-600 hover:bg-orange-700 text-white shadow-lg hover:shadow-xl'
              }`}
          >
            Deinitialize Visualizer
          </button>
        </div>

        {/* Status indicators */}
        <div className="flex gap-4 justify-center mt-6">
          <div className="flex items-center gap-2">
            <div
              className={`w-3 h-3 rounded-full ${isInitialized ? 'bg-green-500' : 'bg-gray-500'
                } animate-pulse`}
            />
            <span className="text-gray-300 text-sm">
              {isInitialized ? 'Initialized' : 'Not Initialized'}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}