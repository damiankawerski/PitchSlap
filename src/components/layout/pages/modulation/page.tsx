import { useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { EffectCard } from "@/components/features/effect-card";
import { EFFECTS_LIST } from "@/lib/consts/effects-list";
import { PageTitle } from "../shared/page-title";
import { get_active_effects, appendEffect, removeEffect } from "@/lib/invokes/modulation";
import { Sliders } from "lucide-react";

// Import images
import amplifierImg from "@/assets/amplifier.jpg";
import autotuneImg from "@/assets/autotune.png";
import bitcrusherImg from "@/assets/bitcrusher.jpg";
import chorusImg from "@/assets/chorus.jpg";
import distortionImg from "@/assets/distortion.jpg";
import pitchshifterImg from "@/assets/pitchshifter.jpg";
import reverbImg from "@/assets/reverb.jpg";
import vibratoImg from "@/assets/vibrato.jpg";
import vocoderImg from "@/assets/vocoder.jpg";
import { Separator } from "@/components/ui/separator";

type EffectData = {
  title: string;
  description: string;
  image: string;
};

const effectsData: Record<string, EffectData> = {
  amplifier: {
    title: "Amplifier",
    description: "Boosts the signal level, adding warmth and volume to your audio input.",
    image: amplifierImg,
  },
  autotune: {
    title: "Auto-Tune",
    description: "Corrects pitch in real-time, smoothing out vocals or creating robotic effects.",
    image: autotuneImg,
  },
  bitcrusher: {
    title: "Bit Crusher",
    description: "Reduces audio fidelity to create a lo-fi, digital distortion effect.",
    image: bitcrusherImg,
  },
  chorus: {
    title: "Chorus",
    description: "Simulates multiple voices by duplicating the signal with slight pitch and timing variations.",
    image: chorusImg,
  },
  distortion: {
    title: "Distortion",
    description: "Alters the original sound by clipping the signal, adding grit and sustain.",
    image: distortionImg,
  },
  pitchshifter: {
    title: "Pitch Shifter",
    description: "Changes the pitch of the audio signal up or down without affecting the duration.",
    image: pitchshifterImg,
  },
  reverb: {
    title: "Reverb",
    description: "Simulates the natural echo of a room, adding space and depth to the sound.",
    image: reverbImg,
  },
  vibrato: {
    title: "Vibrato",
    description: "Modulates the pitch of the signal periodically, creating a pulsing effect.",
    image: vibratoImg,
  },
  vocoder: {
    title: "Vocoder",
    description: "Synthesizes the human voice using an audio signal as a modulator.",
    image: vocoderImg,
  },
};


export default function ModulationPage() {
  const [activeEffects, setActiveEffects] = useState<string[]>([]);
  const navigate = useNavigate();

  useEffect(() => {
    const fetchActiveEffects = () => {
      get_active_effects().then((effects) => {
        if (effects) {
          setActiveEffects((prev) => {
            const isDifferent = 
              effects.length !== prev.length || 
              !effects.every((val, index) => val === prev[index]);
            
            return isDifferent ? effects : prev;
          });
        }
      });
    };

    // Initial fetch
    fetchActiveEffects();
    window.addEventListener("focus", fetchActiveEffects);
    return () => {
      window.removeEventListener("focus", fetchActiveEffects);
    };
  }, []);

  const handleEffectToggle = async (effectKey: string) => {
    const isActive = activeEffects.includes(effectKey);

    try {
      if (isActive) {
        await removeEffect(effectKey);
        setActiveEffects((prev) => prev.filter((e) => e !== effectKey));
      } else {
        await appendEffect(effectKey);
        setActiveEffects((prev) => [...prev, effectKey]);
      }
    } catch (error) {
      console.error("Failed to toggle effect:", error);
    }
  };

  const handleEffectOptions = (effectKey: string) => {
    let routePath = effectKey;
    if (effectKey === 'autotune') routePath = 'auto-tune';
    else if (effectKey === 'bitcrusher') routePath = 'bit-crusher';
    else if (effectKey === 'pitchshifter') routePath = 'pitch-shifter';
    
    navigate({ to: `/modulation/${routePath}` });
  };

  return (
    <div className="container mx-auto p-6 space-y-8">
        <header className="flex flex-col gap-2">
          <PageTitle title="Modulation" icon={Sliders} />
          <p className="text-sm text-muted-foreground">
            Enable and configure real-time audio effects applied to your signal.
          </p>
        </header>
      
      <Separator className="my-4" />

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {EFFECTS_LIST.map((effectKey) => {
          const data = effectsData[effectKey];
          if (!data) return null;
          
          const isActive = activeEffects.includes(effectKey);

          return (
            <EffectCard
              key={effectKey}
              title={data.title}
              description={data.description}
              image={<img src={data.image} alt={data.title} className="w-full h-full object-cover" />}
              active={isActive}
              onToggle={() => handleEffectToggle(effectKey)}
              onOptions={() => handleEffectOptions(effectKey)}
            />
          );
        })}
      </div>
    </div>
  );
}
