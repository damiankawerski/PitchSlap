import { Effects } from "./effects_components/Effects";

export default function EffectsPage() {
    return (
        <div className="min-h-screen p-4 lg:p-8">
            {/* Header section */}
            <div className="mb-8">
                <h1 className="text-3xl lg:text-5xl font-black bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent mb-2">
                    Audio Effects
                </h1>
                <p className="text-gray-400 text-lg">
                    Transform your voice with powerful real-time audio effects
                </p>
                <div className="mt-4 h-1 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 rounded-full w-32 animate-pulse"></div>
            </div>

            {/* Effects grid */}
            <Effects />

            {/* Floating particles effect */}
            <div className="fixed inset-0 pointer-events-none z-0">
                <div className="absolute top-32 left-20 w-2 h-2 bg-purple-500/20 rounded-full animate-ping"></div>
                <div className="absolute top-60 right-32 w-1 h-1 bg-pink-500/30 rounded-full animate-pulse"></div>
                <div className="absolute bottom-40 left-40 w-1.5 h-1.5 bg-orange-500/20 rounded-full animate-bounce"></div>
                <div className="absolute top-80 left-1/2 w-1 h-1 bg-orange-500/20 rounded-full animate-pulse" style={{animationDelay: '1s'}}></div>
                <div className="absolute bottom-80 right-20 w-1.5 h-1.5 bg-purple-500/20 rounded-full animate-bounce" style={{animationDelay: '0.5s'}}></div>
            </div>
        </div>
    );
}