export interface AudioFrame {
  rms: number;
  pitch: number;
  spectrum: number[];
  frequencies: number[];
  timestamp: number;
}