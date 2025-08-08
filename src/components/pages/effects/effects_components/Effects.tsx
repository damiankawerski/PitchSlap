import { useState, useEffect } from 'react';
import { setEffect, getCurrentEffect, clearCurrentEffect } from '../backend_control';
import DemonEffect from './DemonEffect';

export function Effects() {
    const [currentEffect, setCurrentEffect] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(false);

    const onChangeHandler = async (effectName: string) => {
        setIsLoading(true);
        try {
            if (currentEffect === effectName) {
                await clearCurrentEffect();
                setCurrentEffect(null);
            } else {
                await setEffect(effectName);
                setCurrentEffect(effectName);
            }
        } catch (error) {
            console.error(`Error changing effect to ${effectName}:`, error);
        } finally {
            setIsLoading(false);
        }
    }

    const fetchCurrentEffect = async () => {
        const effectName = await getCurrentEffect();
        setCurrentEffect(effectName);
    };

    useEffect(() => {
        fetchCurrentEffect();
    }, [currentEffect, isLoading]);

    return (
        <div className="relative">
            {/* Current Effect Status */}
            <div className="mb-6 bg-gradient-to-r from-gray-800/60 via-gray-700/40 to-gray-900/60 border border-gray-600/30 rounded-2xl p-4 backdrop-blur-sm">
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                        <div className={`w-3 h-3 rounded-full ${currentEffect ? 'bg-green-500 animate-pulse' : 'bg-gray-500'} shadow-lg`}></div>
                        <span className="text-white font-medium">
                            Current Effect: 
                        </span>
                        <span className={`font-bold ${currentEffect ? 'text-green-400' : 'text-gray-400'}`}>
                            {currentEffect || 'None'}
                        </span>
                    </div>
                    {isLoading && (
                        <div className="flex items-center gap-2 text-purple-400">
                            <div className="w-4 h-4 border-2 border-purple-400 border-t-transparent rounded-full animate-spin"></div>
                            <span className="text-sm">Processing...</span>
                        </div>
                    )}
                </div>
            </div>

            {/* Effects Grid - Fixed to align left */}
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 justify-items-start">
                {/* DemonEffect - remove max-width constraint */}
                <div className="w-full max-w-[300px] transform transition-all duration-300 hover:scale-[1.01]">
                    <DemonEffect
                        title="Demon Voice"
                        onChangeHandler={() => onChangeHandler("DemonVoice")}
                        isActive={currentEffect === "DemonVoice"}
                        slug='DemonVoice'
                    />
                </div>

                {/* Placeholder effects - same width constraint */}
                <div className="w-full max-w-[300px] bg-gradient-to-br from-gray-800/60 via-gray-700/40 to-gray-900/60 border border-gray-600/30 rounded-2xl p-4 flex flex-col items-center justify-center min-h-[300px] backdrop-blur-sm relative overflow-hidden group hover:border-purple-500/30 transition-all duration-300">
                    <div className="absolute inset-0 bg-gradient-to-r from-purple-600/5 via-pink-600/5 to-orange-600/5 rounded-2xl blur-xl group-hover:from-purple-600/10 group-hover:via-pink-600/10 group-hover:to-orange-600/10 transition-all duration-300"></div>
                    
                    <div className="relative z-10 text-center">
                        <div className="w-16 h-16 bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-orange-600/20 rounded-full flex items-center justify-center mx-auto mb-4 group-hover:scale-110 transition-transform duration-300">
                            <span className="text-gray-400 text-2xl">🎭</span>
                        </div>
                        <h3 className="text-lg font-bold bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent mb-2">
                            Robot Voice
                        </h3>
                        <p className="text-gray-400 text-sm mb-4">
                            Mechanical robotic voice transformation.
                        </p>
                        <div className="inline-flex items-center gap-2 text-xs text-gray-500 bg-gray-800/50 px-3 py-1 rounded-full">
                            <div className="w-2 h-2 bg-orange-500/60 rounded-full animate-pulse"></div>
                            Coming Soon
                        </div>
                    </div>
                </div>

                <div className="w-full max-w-[300px] bg-gradient-to-br from-gray-800/60 via-gray-700/40 to-gray-900/60 border border-gray-600/30 rounded-2xl p-4 flex flex-col items-center justify-center min-h-[300px] backdrop-blur-sm relative overflow-hidden group hover:border-purple-500/30 transition-all duration-300">
                    <div className="absolute inset-0 bg-gradient-to-r from-purple-600/5 via-pink-600/5 to-orange-600/5 rounded-2xl blur-xl group-hover:from-purple-600/10 group-hover:via-pink-600/10 group-hover:to-orange-600/10 transition-all duration-300"></div>
                    
                    <div className="relative z-10 text-center">
                        <div className="w-16 h-16 bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-orange-600/20 rounded-full flex items-center justify-center mx-auto mb-4 group-hover:scale-110 transition-transform duration-300">
                            <span className="text-gray-400 text-2xl">👻</span>
                        </div>
                        <h3 className="text-lg font-bold bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent mb-2">
                            Echo Chamber
                        </h3>
                        <p className="text-gray-400 text-sm mb-4">
                            Haunting echo effects with spatial reverb.
                        </p>
                        <div className="inline-flex items-center gap-2 text-xs text-gray-500 bg-gray-800/50 px-3 py-1 rounded-full">
                            <div className="w-2 h-2 bg-orange-500/60 rounded-full animate-pulse"></div>
                            Coming Soon
                        </div>
                    </div>
                </div>

                <div className="w-full max-w-[300px] bg-gradient-to-br from-gray-800/60 via-gray-700/40 to-gray-900/60 border border-gray-600/30 rounded-2xl p-4 flex flex-col items-center justify-center min-h-[300px] backdrop-blur-sm relative overflow-hidden group hover:border-purple-500/30 transition-all duration-300">
                    <div className="absolute inset-0 bg-gradient-to-r from-purple-600/5 via-pink-600/5 to-orange-600/5 rounded-2xl blur-xl group-hover:from-purple-600/10 group-hover:via-pink-600/10 group-hover:to-orange-600/10 transition-all duration-300"></div>
                    
                    <div className="relative z-10 text-center">
                        <div className="w-16 h-16 bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-orange-600/20 rounded-full flex items-center justify-center mx-auto mb-4 group-hover:scale-110 transition-transform duration-300">
                            <span className="text-gray-400 text-2xl">🎵</span>
                        </div>
                        <h3 className="text-lg font-bold bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent mb-2">
                            Pitch Shifter
                        </h3>
                        <p className="text-gray-400 text-sm mb-4">
                            Real-time pitch manipulation.
                        </p>
                        <div className="inline-flex items-center gap-2 text-xs text-gray-500 bg-gray-800/50 px-3 py-1 rounded-full">
                            <div className="w-2 h-2 bg-orange-500/60 rounded-full animate-pulse"></div>
                            Coming Soon
                        </div>
                    </div>
                </div>
            </div>

            {/* Quick Actions */}
            <div className="mt-8 flex justify-center">
                <button 
                    onClick={() => {clearCurrentEffect(); setCurrentEffect(null)}}
                    disabled={!currentEffect || isLoading}
                    className={`px-6 py-3 rounded-xl font-semibold transition-all duration-300 transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 ${
                        currentEffect && !isLoading
                            ? 'bg-gradient-to-r from-red-600 to-red-700 hover:from-red-500 hover:to-red-600 text-white shadow-lg shadow-red-500/25 focus:ring-red-500'
                            : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                    }`}
                >
                    <span className="flex items-center gap-2">
                        <span className="w-2 h-2 bg-white rounded-full"></span>
                        Clear All Effects
                    </span>
                </button>
            </div>
        </div>
    );
}