import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import AudioEngineControls from "./AudioEngineControl";

export default function ModulationControl() {
    const [isRunning, setIsRunning] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

    const fetchModulationStatus = async () => {
        try {
            const status = await invoke<boolean>("is_modulation_active");
            setIsRunning(status);
        } catch (error) {
            console.error("Failed to fetch modulation status:", error);
        }
    }

    const toggleModulation = async () => {
        setIsLoading(true);
        try {
            if (!isRunning) {
                await invoke("enable_modulation");
            } else {
                await invoke("disable_modulation");
            }
            setIsRunning(!isRunning);
        } catch (error) {
            console.error("Failed to toggle modulation:", error);
        } finally {
            setIsLoading(false);
        }
    }

    useEffect(() => {
        fetchModulationStatus();
    }, []);

    return (
        <div className="relative group">
            {/* Animated border */}
            <div className="absolute -inset-0.5 bg-gradient-to-r from-purple-600 via-pink-600 to-orange-600 rounded-2xl blur opacity-20 group-hover:opacity-40 transition duration-1000"></div>
            
            {/* Main container */}
            <div className="relative">
                <AudioEngineControls
                    onChangeHandler={toggleModulation}
                    title="🔄 Audio Modulation"
                    description="Set up and control audio modulation effects. Toggle modulation on or off to apply effects to your audio stream."
                    isRunning={isRunning}
                    startButtonText={isLoading ? "Starting..." : "Start Modulation"}
                    stopButtonText={isLoading ? "Stopping..." : "Stop Modulation"}
                />
            </div>

            {/* Floating info badge */}
            {isRunning && (
                <div className="absolute -top-2 -right-2 bg-gradient-to-r from-green-500 to-emerald-500 text-white text-xs font-bold px-3 py-1 rounded-full shadow-lg animate-bounce">
                    LIVE
                </div>
            )}
        </div>
    );
}