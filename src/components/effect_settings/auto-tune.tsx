import { useEffect, useRef, useState } from 'react';
import { CustomInputNumber } from '@/components/controls/custom-input-number';
import { get_parameters, set_effect_parameters } from '@/lib/invokes/modulation';
import { set_auto_tune_scale } from '@/lib/invokes/modulation';
import { CommonSettingsSelector } from '@/components/controls/selectors/common-settings-select';
import { Music } from 'lucide-react';

const EFFECT_NAME = 'autotune';

const SCALES = [
  'CMajor',
  'AMajor',
  'GMajor',
  'DMajor',
  'EMajor',
  'FMajor',
  'GMinor',
  'DMinor',
  'AMinor',
  'EMinor',
];

const PARAMS = [
  { name: 'correction_speed', label: 'Correction Speed', min: 0, max: 1, step: 0.01, default: 0.95 },
  { name: 'detection_window_size', label: 'Detection Window Size', min: 128, max: 4096, step: 128, default: 1024 },
  { name: 'power_threshold', label: 'Power Threshold', min: 0, max: 1, step: 0.01, default: 0.05 },
  { name: 'clarity_threshold', label: 'Clarity Threshold', min: 0, max: 1, step: 0.01, default: 0.3 },
] as const;

type ParamName = (typeof PARAMS)[number]['name'];

export function AutoTuneSettings() {
  const [scale, setScale] = useState('CMajor');
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

  const handleScaleChange = (value: string) => {
    setScale(value);
    set_auto_tune_scale(value);
  };

  return (
    <div className="flex flex-col gap-6">
      <CommonSettingsSelector
        label="Scale"
        items={SCALES}
        value={scale}
        onChange={handleScaleChange}
        placeholder="Select a scale"
        icon={Music}
      />
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
