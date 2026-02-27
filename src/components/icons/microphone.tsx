import { Mic, MicOff } from 'lucide-react';

export function MicrophoneIcon({ enabled }: { enabled: boolean }) {
  return (
    <div className="relative w-8 h-8 flex items-center justify-center">
      {enabled ? (
        <div className="relative">
          <Mic 
            size={24} 
            className="text-rose-400 drop-shadow-[0_0_8px_rgba(251,113,133,0.5)] transition-all duration-300"
          />
          <div className="absolute inset-0 animate-ping">
            <Mic 
              size={24} 
              className="text-rose-400 opacity-30"
            />
          </div>
        </div>
      ) : (
        <MicOff 
          size={24} 
          className="text-gray-500 transition-all duration-300"
        />
      )}
    </div>
  );
}