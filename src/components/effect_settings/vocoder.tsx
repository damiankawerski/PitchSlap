import { useEffect, useRef, useState } from 'react';
import { CustomInputNumber } from '@/components/controls/custom-input-number';
import { get_parameters, set_effect_parameters } from '@/lib/invokes/modulation';

const EFFECT_NAME = 'vocoder';

const PARAMS = [
  { name: 'band_count', label: 'Band Count', min: 1, max: 64, step: 1, default: 32 },
  { name: 'min_freq', label: 'Min Frequency (Hz)', min: 20, max: 4000, step: 1, default: 90 },
  { name: 'max_freq', label: 'Max Frequency (Hz)', min: 1000, max: 20000, step: 10, default: 9000 },
  { name: 'q', label: 'Q Factor', min: 0.5, max: 30, step: 0.1, default: 9 },
  { name: 'attack_ms', label: 'Attack (ms)', min: 0.1, max: 200, step: 0.1, default: 2 },
  { name: 'release_ms', label: 'Release (ms)', min: 1, max: 500, step: 1, default: 50 },
  { name: 'output_gain', label: 'Output Gain', min: 0, max: 20, step: 0.1, default: 4 },
  { name: 'dry_mix', label: 'Dry Mix', min: 0, max: 1, step: 0.01, default: 0.2 },
  { name: 'env_gain', label: 'Envelope Gain', min: 0, max: 30, step: 0.1, default: 11 },
  { name: 'env_floor', label: 'Envelope Floor', min: 0, max: 0.1, step: 0.0001, default: 0.0015 },
  { name: 'soft_clip', label: 'Soft Clip', min: 0, max: 4, step: 0.01, default: 1.6 },
  { name: 'reverb_mix', label: 'Reverb Mix', min: 0, max: 1, step: 0.01, default: 0.08 },
  { name: 'carrier_base_freq', label: 'Carrier Base Freq (Hz)', min: 20, max: 2000, step: 1, default: 110 },
  { name: 'carrier_harmonics', label: 'Carrier Harmonics', min: 1, max: 64, step: 1, default: 18 },
  { name: 'carrier_gain', label: 'Carrier Gain', min: 0, max: 1, step: 0.01, default: 0.22 },
] as const;

type ParamName = (typeof PARAMS)[number]['name'];

export function VocoderSettings() {
  const [values, setValues] = useState<Record<ParamName, number>>(
    Object.fromEntries(PARAMS.map((p) => [p.name, p.default])) as Record<ParamName, number>,
  );
  const timers = useRef<Partial<Record<ParamName, ReturnType<typeof setTimeout>>>>({});

  useEffect(() => {
    get_parameters(EFFECT_NAME).then((params) => {
      if (!params.length) return;
      setValues((prev) => {
        const next = { ...prev };
        for (const p of params) {
          if (p.name in next) (next as Record<string, number>)[p.name] = p.value;
        }
        return next;
      });
    });
  }, []);

  const handleChange = (name: ParamName, value: number | null) => {
    if (value === null) return;
    setValues((prev) => ({ ...prev, [name]: value }));
    clearTimeout(timers.current[name]);
    timers.current[name] = setTimeout(() => {
      set_effect_parameters(EFFECT_NAME, name, value);
    }, 350);
  };

  return (
    <div className="flex flex-col gap-6">
      {PARAMS.map((p) => (
        <CustomInputNumber
          key={p.name}
          label={p.label}
          min={p.min}
          max={p.max}
          step={p.step}
          value={values[p.name]}
          onChange={(v) => handleChange(p.name, v)}
        />
      ))}
    </div>
  );
}
