import { useEffect, useRef, useState } from 'react';
import { CustomInputNumber } from '@/components/controls/custom-input-number';
import { get_parameters, set_effect_parameters } from '@/lib/invokes/modulation';
import { TransparentCard } from '../ui/transparent-card';

const EFFECT_NAME = 'distortion';

const PARAMS = [
  { name: 'gain', label: 'Gain', min: 0, max: 50, step: 0.1, default: 1 },
] as const;

type ParamName = (typeof PARAMS)[number]['name'];

export function DistortionSettings() {
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
    <TransparentCard className="pt-0 pb-6">
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
    </TransparentCard>
  );
}
