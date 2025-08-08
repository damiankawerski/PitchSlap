import LoopbackControl from "./AudioControls/LoopbackControl";
import ModulationControl from "./AudioControls/ModulationControl";
import ThroughputControl from "./AudioControls/ThroughputControl";

export default function HomePage() {
    return (
        <div className="max-h-screen p-4 lg:p-8">
            {/* Header section */}
            <div className="mb-8">
                <h1 className="text-3xl lg:text-5xl font-black bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent mb-2">
                    Home
                </h1>
                <p className="text-gray-400 text-lg">
                    Control and monitor your audio processing engines
                </p>
                <div className="mt-4 h-1 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 rounded-full w-24 animate-pulse"></div>
            </div>

            {/* Controls grid */}
            <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
                <div className="transform transition-all duration-300 hover:scale-[1.02]">
                    <LoopbackControl />
                </div>

                <div className="transform transition-all duration-300 hover:scale-[1.02]">
                    <ThroughputControl />
                </div>

                <div className="transform transition-all duration-300 hover:scale-[1.02]">
                    <ModulationControl />
                </div>
                
                
                {/* Placeholder for future controls */}
                <div className="bg-gray-800/50 border border-gray-700/50 rounded-2xl p-6 flex items-center justify-center min-h-[200px] backdrop-blur-sm">
                    <div className="text-center">
                        <div className="w-12 h-12 bg-gradient-to-r from-purple-600/20 to-pink-600/20 rounded-full flex items-center justify-center mx-auto mb-3">
                            <span className="text-gray-500 text-xl">⚙️</span>
                        </div>
                        <p className="text-gray-500 text-sm">More controls coming soon...</p>
                    </div>
                </div>
            </div>
        </div>
    );
}