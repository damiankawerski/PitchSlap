import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import AudioEngineControls from "./AudioEngineControl";

export default function LoopbackControl() {
    const [isRunning, setIsRunning] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

    const fetchLoopbackStatus = async () => {
        try {
            const status = await invoke<boolean>("is_loopback_running");
            setIsRunning(status);
        } catch (error) {
            console.error("Failed to fetch loopback status:", error);
        }
    }

    const toggleLoopback = async () => {
        setIsLoading(true);
        try {
            if (isRunning) {
                await invoke("stop_loopback");
            } else {
                await invoke("loopback");
            }
            setIsRunning(!isRunning);
        } catch (error) {
            console.error("Failed to toggle loopback:", error);
        } finally {
            setIsLoading(false);
        }
    }

    useEffect(() => {
        fetchLoopbackStatus();
    }, []);

    return (
        <div className="relative group">
            {/* Animated border */}
            <div className="absolute -inset-0.5 bg-gradient-to-r from-purple-600 via-pink-600 to-orange-600 rounded-2xl blur opacity-20 group-hover:opacity-40 transition duration-1000"></div>
            
            {/* Main container */}
            <div className="relative">
                <AudioEngineControls
                    onChangeHandler={toggleLoopback}
                    title="🔄 Audio Loopback"
                    description="Direct audio loopback from input to output with real-time processing. Perfect for live monitoring and effects application."
                    isRunning={isRunning}
                    startButtonText={isLoading ? "Starting..." : "Start Loopback"}
                    stopButtonText={isLoading ? "Stopping..." : "Stop Loopback"}
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