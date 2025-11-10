import { Tooltip, TooltipContent, TooltipTrigger, TooltipProvider } from "@/components/ui/tooltip";
import { useState, useEffect } from "react";
import { startLoopbackInvoke, stopLoopbackInvoke } from "@/lib/invokes/loopback";
import { startThroughputInvoke, stopThroughputInvoke } from "@/lib/invokes/throughput";
import { enableModulationInvoke, disableModulationInvoke, isModulationEnabled } from "@/lib/invokes/modulation";
import { isLoopbackRunning, isThroughputRunning } from "@/lib/invokes/config-getters";
import { EffectsIcon } from "../icons/effect";
import { MicrophoneIcon } from "../icons/microphone";
import { LoopbackIcon } from "../icons/loopback";
import { Button } from "@/components/ui/button";

export function QuickActions() {
  const [microphoneEnabled, setMicrophoneEnabled] = useState(false);
  const [loopbackEnabled, setLoopbackEnabled] = useState(false);
  const [effectsEnabled, setEffectsEnabled] = useState(false);

  useEffect(() => {
    const initStates = async () => {
      const [throughput, loopback, modulation] = await Promise.all([
        isThroughputRunning(),
        isLoopbackRunning(),
        isModulationEnabled()
      ]);
      setMicrophoneEnabled(throughput);
      setLoopbackEnabled(loopback);
      setEffectsEnabled(modulation);
    };
    initStates();
  }, []);

  useEffect(() => {
    if (loopbackEnabled) startLoopbackInvoke();
    else stopLoopbackInvoke();
  }, [loopbackEnabled]);

  useEffect(() => {
    if (effectsEnabled) enableModulationInvoke();
    else disableModulationInvoke();
  }, [effectsEnabled]);

  useEffect(() => {
    if (microphoneEnabled) startThroughputInvoke();
    else stopThroughputInvoke();
  }, [microphoneEnabled]);

  return (
    <TooltipProvider>
      <div className="flex flex-row items-center gap-4">
        {/* Microphone */}
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant="sidebar-action"
              size="icon"
              onClick={() => setMicrophoneEnabled(!microphoneEnabled)}
            >
              <MicrophoneIcon enabled={microphoneEnabled} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{microphoneEnabled ? "Disable microphone" : "Enable microphone"}</p>
          </TooltipContent>
        </Tooltip>

        {/* Loopback */}
        <Tooltip>
          <TooltipTrigger >
            <Button
              variant="sidebar-action"
              size="icon"
              onClick={() => setLoopbackEnabled(!loopbackEnabled)}
            >
              <LoopbackIcon enabled={loopbackEnabled} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{loopbackEnabled ? "Disable loopback" : "Enable loopback"}</p>
          </TooltipContent>
        </Tooltip>

        {/* Effects */}
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant="sidebar-action"
              size="icon"
              onClick={() => setEffectsEnabled(!effectsEnabled)}
            >
              <EffectsIcon enabled={effectsEnabled} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{effectsEnabled ? "Disable effects" : "Enable effects"}</p>
          </TooltipContent>
        </Tooltip>

        
      </div>
    </TooltipProvider>
  );
}
