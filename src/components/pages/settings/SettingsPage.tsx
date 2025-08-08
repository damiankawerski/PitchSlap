import InputDevice from "./InputDevice";
import OutputDevice from "./OutputDevice";
import VirtualDevice from "./VirtualDevice";
import Latency from "./Latency";

export default function SettingsPage() {
    return (
        <div className="text-white p-4">
            {/* Enhanced Header */}
            <div className="relative mb-8">
                {/* Background glow effect */}
                <div className="absolute inset-0 bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-orange-600/20 blur-3xl"></div>
                
                {/* Main title */}
                <div className="relative text-center">
                    <h1 className="text-5xl lg:text-6xl font-black tracking-wider bg-gradient-to-r from-purple-400 via-pink-400 via-blue-400 to-orange-400 bg-clip-text text-transparent drop-shadow-2xl mb-2">
                        ⚙️ SETTINGS
                    </h1>
                    
                    {/* Subtitle */}
                    <p className="text-xl text-gray-300 font-light tracking-widest">
                        CONFIGURE YOUR AUDIO STUDIO
                    </p>
                    
                    {/* Animated underline */}
                    <div className="mt-4 mx-auto w-32 h-1 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 rounded-full animate-pulse"></div>
                    
                    {/* Decorative elements */}
                    <div className="flex justify-center items-center gap-4 mt-4">
                        <div className="w-2 h-2 bg-purple-500 rounded-full animate-pulse"></div>
                        <div className="w-2 h-2 bg-pink-500 rounded-full animate-pulse" style={{animationDelay: '0.5s'}}></div>
                        <div className="w-2 h-2 bg-orange-500 rounded-full animate-pulse" style={{animationDelay: '1s'}}></div>
                    </div>
                </div>
            </div>

            {/* Settings Cards */}
            <div className="space-y-6">
                <InputDevice />
                <OutputDevice />
                <VirtualDevice />
                <Latency />
            </div>
        </div>
    );
}