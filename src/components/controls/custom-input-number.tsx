


import { Input } from "@/components/ui/input";

interface CustomInputNumberProps {
  label?: string;
  placeholder?: string;
  onChange?: (value: number | null) => void;
  value?: number | "";
  defaultValue?: number;
  disabled?: boolean;
  className?: string;
  min?: number;
  max?: number;
  step?: number;
}

export function CustomInputNumber({
  label,
  placeholder = "Enter a value",
  onChange,
  value,
  defaultValue,
  disabled = false,
  className = "",
  min,
  max,
  step,
}: CustomInputNumberProps) {
  return (
    <div className={`flex flex-col gap-3 w-full ${className}`}>
      {label && (
        <label className="text-sm font-semibold text-foreground/90 tracking-wide transition-colors duration-200">
          {label}
        </label>
      )}

      <div className="relative w-full group">
        <div className="absolute inset-0 rounded-lg bg-gradient-to-r from-purple-500/20 via-blue-500/20 to-cyan-500/20 blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
        <div className="absolute inset-0 rounded-lg bg-gradient-to-r from-purple-500/10 via-blue-500/10 to-cyan-500/10 animate-pulse" />

        <Input
          type="number"
          inputMode="decimal"
          placeholder={placeholder}
          disabled={disabled}
          min={min}
          max={max}
          step={step}
          value={value}
          defaultValue={defaultValue}
          onChange={(e) => {
            const raw = e.currentTarget.value;
            if (!onChange) return;
            onChange(raw === "" ? null : Number(raw));
          }}
          className="relative h-11 w-full backdrop-blur-md bg-card/60 border-2 border-transparent bg-gradient-to-r from-purple-500/30 via-blue-500/30 to-cyan-500/30 bg-clip-padding hover:border-purple-500/50 transition-all duration-300 shadow-lg hover:shadow-xl hover:shadow-purple-500/20"
        />
      </div>
    </div>
  );
}