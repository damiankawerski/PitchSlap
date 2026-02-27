import { AudioWaveform, Repeat } from 'lucide-react';

export function LoopbackIcon({ enabled }: { enabled: boolean }) {
  return (
    <div className="relative w-8 h-8 flex items-center justify-center">
      <AudioWaveform 
        size={24} 
        className={`transition-all duration-300 ${
          enabled 
            ? 'text-blue-400 drop-shadow-[0_0_8px_rgba(96,165,250,0.5)]' 
            : 'text-gray-500'
        }`}
      />
      {enabled && (
        <Repeat 
          size={10} 
          className="absolute -bottom-0.5 -right-0.5 text-purple-400 animate-spin [animation-duration:3s] drop-shadow-[0_0_4px_rgba(192,132,252,0.8)]"
        />
      )}
    </div>
  );
}