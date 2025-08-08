import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import OptionCard from "./SettingOption/OptionCard";

const latencyOptions = [
    { value: 20, label: "20 ms", description: "Ultra Low - Minimal delay, high CPU usage" },
    { value: 50, label: "50 ms", description: "Very Low - Good balance for most users" },
    { value: 100, label: "100 ms", description: "Low - Stable with good performance" },
    { value: 150, label: "150 ms", description: "Medium - Recommended for most systems" },
    { value: 200, label: "200 ms", description: "High - More stable, less CPU usage" },
    { value: 250, label: "250 ms", description: "Very High - Maximum stability" }
];

const TimerSvg = () => {
    return (
        <svg fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm1-13h-2v6l5.25 3.15.75-1.23-4.5-2.67V7z"/>
        </svg>
    );
}

export default function Latency() {
    const [currentLatency, setCurrentLatency] = useState<string>("");
    const [availableLatencies, setAvailableLatencies] = useState<string[]>([]);

    const fetchCurrentLatency = async () => {
        try {
            const latency = await invoke<number>("get_latency");
            const latencyOption = latencyOptions.find(option => option.value === latency);
            if (latencyOption) {
                setCurrentLatency(`${latencyOption.label} - ${latencyOption.description}`);
            } else {
                setCurrentLatency(`${latency} ms - Custom value`);
            }
        } catch (error) {
            console.error("Failed to fetch current latency:", error);
        }
    };

    const fetchAvailableLatencies = async () => {
        setAvailableLatencies(latencyOptions.map(option => `${option.label} - ${option.description}`));
    }

    const handleLatencyChange = async (latencyLabel: string) => {
        try {
            const labelPart = latencyLabel.split(' - ')[0];
            const latencyOption = latencyOptions.find(option => option.label === labelPart);
            if (latencyOption) {
                await invoke("set_latency", { latency: latencyOption.value });
                setCurrentLatency(latencyLabel);
                
                setTimeout(() => {
                    fetchCurrentLatency();
                }, 100);
            } else {
                console.error("Unknown latency option:", latencyLabel);
            }
        } catch (error) {
            console.error("Failed to set latency:", error);
            fetchCurrentLatency(); // Refresh state on error
        }
    };

    useEffect(() => {
        fetchAvailableLatencies();
        fetchCurrentLatency();
    }, []);

    return (
        <OptionCard
            title="Latency"
            description="Select your audio latency"
            icon={<TimerSvg />}
            currentOption={currentLatency}
            availableOptions={availableLatencies}
            onOptionChange={handleLatencyChange}
        />
    );
}