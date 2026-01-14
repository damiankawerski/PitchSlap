declare module "@/components/bits/DarkVeil" {
  import type { ComponentType } from "react";

  const DarkVeil: ComponentType<{
    hueShift?: number;
    noiseIntensity?: number;
    scanlineIntensity?: number;
    speed?: number;
    scanlineFrequency?: number;
    warpAmount?: number;
    resolutionScale?: number;
  }>;

  export default DarkVeil;
}
