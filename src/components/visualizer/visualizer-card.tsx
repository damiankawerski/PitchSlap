import { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import {
  isVisualizerInitializedInvoke,
  initializeVisualizerInvoke,
  deinitializeVisualizerInvoke,
} from '../../lib/invokes/visualizer';
import { listenVisualizer } from '../../lib/utils/listener';
import { AudioFrame } from '../../lib/utils/audio-frame';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

const CANVAS_W = 1200;
const CANVAS_H = 320;
const PEAK_DECAY = 0.012;

const FILL_VERT = /* glsl */ `
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`;

// Poziomy gradient kolorów (bas → sopran) × pionowe wygaszanie (dół przezroczysty)
const FILL_FRAG = /* glsl */ `
  varying vec2 vUv;
  void main() {
    vec3 c0 = vec3(0.655, 0.545, 0.980); // violet-400
    vec3 c1 = vec3(0.506, 0.549, 0.949); // indigo-400
    vec3 c2 = vec3(0.204, 0.827, 0.600); // emerald-400
    float t = vUv.x;
    vec3 col = t < 0.5
      ? mix(c0, c1, t * 2.0)
      : mix(c1, c2, (t - 0.5) * 2.0);
    float alpha = vUv.y * 0.65 * smoothstep(0.0, 0.025, vUv.y);
    gl_FragColor = vec4(col, alpha);
  }
`;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

export function AudioSpectrumVisualizer() {
  const mountRef = useRef<HTMLDivElement>(null);

  // Three.js refs
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const sceneRef    = useRef<THREE.Scene | null>(null);
  const cameraRef   = useRef<THREE.OrthographicCamera | null>(null);

  // Dynamic geometry refs – aktualizowane każdą ramkę
  const fillPosAttrRef = useRef<THREE.BufferAttribute | null>(null);
  const linePosAttrRef = useRef<THREE.BufferAttribute | null>(null);
  const peakPosAttrRef = useRef<THREE.BufferAttribute | null>(null);
  const numBinsRef     = useRef(0);

  // Audio / state refs
  const peakHoldRef  = useRef<number[]>([]);
  const frameCountRef = useRef(0);
  const lastFpsRef   = useRef(Date.now());
  const latestFrameRef = useRef<AudioFrame | null>(null);
  const rafRef       = useRef<number | null>(null);

  const [isInitialized, setIsInitialized] = useState(false);
  const [fps, setFps]         = useState(0);
  const [peakHz, setPeakHz]   = useState(0);
  const [rmsLevel, setRmsLevel] = useState(0);

  useEffect(() => {
    isVisualizerInitializedInvoke().then(setIsInitialized);
  }, []);

  // ---------------------------------------------------------------------------
  // Three.js init
  // ---------------------------------------------------------------------------
  useEffect(() => {
    if (!mountRef.current) return;

    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: false });
    renderer.setSize(CANVAS_W, CANVAS_H);
    renderer.setPixelRatio(1);
    renderer.setClearColor(0x09090b, 1);
    renderer.domElement.style.width  = '100%';
    renderer.domElement.style.height = 'auto';
    renderer.domElement.style.display = 'block';
    mountRef.current.appendChild(renderer.domElement);
    rendererRef.current = renderer;

    // OrthographicCamera: left=0, right=W, top=H, bottom=0 (pixel space, Y↑)
    const camera = new THREE.OrthographicCamera(0, CANVAS_W, CANVAS_H, 0, -1, 1);
    cameraRef.current = camera;

    const scene = new THREE.Scene();
    sceneRef.current = scene;

    // --- Grid lines ---
    [0.25, 0.5, 0.75].forEach(l => {
      const y = l * CANVAS_H * 0.88;
      const geo = new THREE.BufferGeometry().setFromPoints([
        new THREE.Vector3(0, y, 0.05),
        new THREE.Vector3(CANVAS_W, y, 0.05),
      ]);
      scene.add(new THREE.Line(geo,
        new THREE.LineBasicMaterial({ color: 0xffffff, transparent: true, opacity: 0.04 })
      ));
    });

    return () => {
      renderer.dispose();
      mountRef.current?.removeChild(renderer.domElement);
      rendererRef.current = null;
      sceneRef.current = null;
    };
  }, []);

  // ---------------------------------------------------------------------------
  // Tworzy/wymienia dynamiczne geometrie gdy N binów się zmieni
  // ---------------------------------------------------------------------------
  const initDynamicGeo = (scene: THREE.Scene, N: number) => {
    // Usuń stare dynamiczne obiekty
    const toRemove = scene.children.filter((c: THREE.Object3D) => c.userData.dyn);
    toRemove.forEach((c: THREE.Object3D) => scene.remove(c));

    numBinsRef.current = N;

    // --- Fill mesh: 2 werteksy na bin (dół y=0, góra y=value) ---
    // Indeksy: (N-1) × 2 trójkąty = (N-1) × 6 indeksów
    const fillPos = new Float32Array(N * 2 * 3);
    const fillUv  = new Float32Array(N * 2 * 2);
    const fillIdx = new Uint32Array((N - 1) * 6);

    for (let i = 0; i < N; i++) {
      const x  = (i / (N - 1)) * CANVAS_W;
      const u  = i / (N - 1);
      // bottom vertex [i*2]
      fillPos.set([x, 0, 0],      i * 6);
      fillUv.set([u, 0],          i * 4);
      // top vertex [i*2+1]  – y aktualizowany co ramkę
      fillPos.set([x, 0, 0],      i * 6 + 3);
      fillUv.set([u, 1],          i * 4 + 2);
    }
    for (let i = 0; i < N - 1; i++) {
      const bl = i * 2, br = (i + 1) * 2, tl = bl + 1, tr = br + 1;
      fillIdx.set([bl, br, tl, tl, br, tr], i * 6);
    }

    const fillGeo = new THREE.BufferGeometry();
    const fillPosAttr = new THREE.BufferAttribute(fillPos, 3);
    fillPosAttr.setUsage(THREE.DynamicDrawUsage);
    fillGeo.setAttribute('position', fillPosAttr);
    fillGeo.setAttribute('uv', new THREE.BufferAttribute(fillUv, 2));
    fillGeo.setIndex(new THREE.BufferAttribute(fillIdx, 1));
    fillPosAttrRef.current = fillPosAttr;

    const fillMesh = new THREE.Mesh(fillGeo, new THREE.ShaderMaterial({
      vertexShader: FILL_VERT,
      fragmentShader: FILL_FRAG,
      transparent: true,
      depthWrite: false,
    }));
    fillMesh.userData.dyn = true;
    scene.add(fillMesh);

    // --- Outline line ---
    const linePos = new Float32Array(N * 3);
    const lineGeo = new THREE.BufferGeometry();
    const linePosAttr = new THREE.BufferAttribute(linePos, 3);
    linePosAttr.setUsage(THREE.DynamicDrawUsage);
    lineGeo.setAttribute('position', linePosAttr);
    linePosAttrRef.current = linePosAttr;

    const outlineLine = new THREE.Line(lineGeo,
      new THREE.LineBasicMaterial({ color: 0xa78bfa, transparent: true, opacity: 0.9 })
    );
    outlineLine.userData.dyn = true;
    scene.add(outlineLine);

    // --- Peak-hold line ---
    const peakPos = new Float32Array(N * 3);
    const peakGeo = new THREE.BufferGeometry();
    const peakPosAttr = new THREE.BufferAttribute(peakPos, 3);
    peakPosAttr.setUsage(THREE.DynamicDrawUsage);
    peakGeo.setAttribute('position', peakPosAttr);
    peakPosAttrRef.current = peakPosAttr;

    const peakLine = new THREE.Line(peakGeo,
      new THREE.LineBasicMaterial({ color: 0xfbbf24, transparent: true, opacity: 0.5 })
    );
    peakLine.userData.dyn = true;
    scene.add(peakLine);
  };

  // ---------------------------------------------------------------------------
  // Draw loop
  // ---------------------------------------------------------------------------
  const draw = () => {
    const scene    = sceneRef.current;
    const camera   = cameraRef.current;
    const renderer = rendererRef.current;
    if (!scene || !camera || !renderer) { rafRef.current = requestAnimationFrame(draw); return; }

    const frame = latestFrameRef.current;
    if (!frame) { renderer.render(scene, camera); rafRef.current = requestAnimationFrame(draw); return; }

    const bins = frame.spectrum;
    const N    = bins.length;

    if (numBinsRef.current !== N) initDynamicGeo(scene, N);

    // Peak hold
    if (peakHoldRef.current.length !== N) peakHoldRef.current = new Array(N).fill(0);
    const peaks = peakHoldRef.current;
    for (let i = 0; i < N; i++) {
      peaks[i] = bins[i] >= peaks[i] ? bins[i] : Math.max(0, peaks[i] - PEAK_DECAY);
    }

    // Update fill – tylko werteksy na górze (co drugi, zaczynając od idx 1)
    const fillArr = fillPosAttrRef.current?.array as Float32Array | undefined;
    if (fillArr) {
      for (let i = 0; i < N; i++) {
        fillArr[i * 6 + 4] = bins[i] * CANVAS_H * 0.88; // top y
      }
      fillPosAttrRef.current!.needsUpdate = true;
    }

    // Update outline
    const lineArr = linePosAttrRef.current?.array as Float32Array | undefined;
    if (lineArr) {
      for (let i = 0; i < N; i++) {
        lineArr[i * 3 + 0] = (i / (N - 1)) * CANVAS_W;
        lineArr[i * 3 + 1] = bins[i] * CANVAS_H * 0.88;
        lineArr[i * 3 + 2] = 0.1;
      }
      linePosAttrRef.current!.needsUpdate = true;
    }

    // Update peak
    const peakArr = peakPosAttrRef.current?.array as Float32Array | undefined;
    if (peakArr) {
      for (let i = 0; i < N; i++) {
        peakArr[i * 3 + 0] = (i / (N - 1)) * CANVAS_W;
        peakArr[i * 3 + 1] = peaks[i] * CANVAS_H * 0.88;
        peakArr[i * 3 + 2] = 0.2;
      }
      peakPosAttrRef.current!.needsUpdate = true;
    }

    renderer.render(scene, camera);

    // FPS & stats (1s interval)
    frameCountRef.current++;
    const now = Date.now();
    if (now - lastFpsRef.current >= 1000) {
      let maxVal = 0, peakIdx = 0;
      for (let i = 0; i < N; i++) { if (bins[i] > maxVal) { maxVal = bins[i]; peakIdx = i; } }
      setFps(frameCountRef.current);
      setPeakHz(Math.round(frame.frequencies[peakIdx] ?? 0));
      setRmsLevel(Math.round(frame.rms * 100));
      frameCountRef.current = 0;
      lastFpsRef.current = now;
    }

    rafRef.current = requestAnimationFrame(draw);
  };

  useEffect(() => {
    rafRef.current = requestAnimationFrame(draw);
    return () => { if (rafRef.current !== null) cancelAnimationFrame(rafRef.current); };
  }, []);

  useEffect(() => {
    return listenVisualizer(isInitialized, frame => { latestFrameRef.current = frame; });
  }, [isInitialized]);

  return (
    <div className="flex flex-col gap-4 p-4 w-full">
      <Card className="border-zinc-800 bg-zinc-950/80 backdrop-blur-sm">
        <CardHeader className="pb-3 flex flex-row items-center justify-between">
          <CardTitle className="text-base font-semibold text-zinc-100 tracking-wide">
            Spectrum Analyzer
          </CardTitle>
          <div className="flex items-center gap-4 font-mono text-xs text-zinc-400">
            <span><span className="text-zinc-600">RMS </span><span className="text-zinc-200">{rmsLevel}%</span></span>
            <span><span className="text-zinc-600">PEAK </span><span className="text-zinc-200">{peakHz} Hz</span></span>
            <span><span className="text-zinc-600">FPS </span><span className={fps >= 30 ? 'text-emerald-400' : 'text-amber-400'}>{fps}</span></span>
          </div>
        </CardHeader>

        <CardContent className="flex flex-col gap-4 p-4 pt-0">
          <div
            ref={mountRef}
            className="relative w-full overflow-hidden rounded-md"
            style={{ boxShadow: 'inset 0 0 0 1px rgba(255,255,255,0.06)' }}
          >
            {/* Three.js canvas wstrzykuje się tutaj przez mountRef */}
            <div className="absolute bottom-3 left-3 flex items-center gap-1.5 z-10 pointer-events-none">
              <span className={`w-2 h-2 rounded-full ${isInitialized ? 'bg-emerald-500' : 'bg-zinc-600'} shadow-sm`} />
              <span className="text-[10px] text-zinc-500 font-mono uppercase tracking-widest">
                {isInitialized ? 'live' : 'idle'}
              </span>
            </div>
          </div>

          <div className="flex gap-3">
            <Button
              size="sm"
              variant={isInitialized ? 'outline' : 'default'}
              disabled={isInitialized}
              onClick={() => initializeVisualizerInvoke().then(() => setIsInitialized(true))}
              className="flex-1 text-xs"
            >
              Initialize Audio
            </Button>
            <Button
              size="sm"
              variant="outline"
              disabled={!isInitialized}
              onClick={() => deinitializeVisualizerInvoke().then(() => setIsInitialized(false))}
              className="flex-1 text-xs text-zinc-400 border-zinc-700 hover:bg-zinc-800 hover:text-zinc-200 disabled:opacity-30"
            >
              Stop
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

