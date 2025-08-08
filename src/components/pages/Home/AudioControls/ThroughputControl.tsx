import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import AudioEngineControls from "./AudioEngineControl";

export default function ThroughputControl() {
    const [isRunning, setIsRunning] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

    const fetchThroughputStatus = async () => {
        try {
            const status = await invoke<boolean>("is_throughput_running");
            setIsRunning(status);
        } catch (error) {
            console.error("Failed to fetch throughput status:", error);
        }
    }

    const toggleThroughput = async () => {
        setIsLoading(true);
        try {
            if (isRunning) {
                await invoke("stop_throughput");
            } else {
                await invoke("throughput");
            }
            setIsRunning(!isRunning);
        } catch (error) {
            console.error("Failed to toggle throughput:", error);
        } finally {
            setIsLoading(false);
        }
    }

    useEffect(() => {
        fetchThroughputStatus();
    }, []);

    return (
        <div className="relative group">
            {/* Animated border */}
            <div className="absolute -inset-0.5 bg-gradient-to-r from-purple-600 via-pink-600 to-orange-600 rounded-2xl blur opacity-20 group-hover:opacity-40 transition duration-1000"></div>
            
            {/* Main container */}
            <div className="relative">
                <AudioEngineControls
                    onChangeHandler={toggleThroughput}
                    title="🔄 Audio Throughput"
                    description="Direct audio throughput from input to virtual output with real-time processing. Perfect for live monitoring and effects application."
                    isRunning={isRunning}
                    startButtonText={isLoading ? "Starting..." : "Start Throughput"}
                    stopButtonText={isLoading ? "Stopping..." : "Stop Throughput"}
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