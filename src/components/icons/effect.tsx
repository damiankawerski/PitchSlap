import { Sliders, Zap } from 'lucide-react';

export function EffectsIcon({ enabled }: { enabled: boolean }) {
  return (
    <div className="relative w-8 h-8 flex items-center justify-center">
      <Sliders 
        size={24} 
        className={`transition-all duration-300 ${
          enabled 
            ? 'text-emerald-400 drop-shadow-[0_0_8px_rgba(52,211,153,0.5)]' 
            : 'text-gray-500'
        }`}
      />
      {enabled && (
        <Zap 
          size={10} 
          className="absolute -top-0.5 -right-0.5 text-amber-400 animate-pulse drop-shadow-[0_0_4px_rgba(251,191,36,0.8)]"
        />
      )}
    </div>
  );
}