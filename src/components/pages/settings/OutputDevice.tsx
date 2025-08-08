import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import OptionCard from "./SettingOption/OptionCard";

const HeadphonesSvg = () => {
    return (
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 1a9 9 0 00-9 9v7c0 1.66 1.34 3 3 3h1v-8H6v-2c0-3.31 2.69-6 6-6s6 2.69 6 6v2h-1v8h1c1.66 0 3-1.34 3-3v-7a9 9 0 00-9-9zM8 15v3H6c-.55 0-1-.45-1-1v-2h3zm10 3v-3h3v2c0 .55-.45 1-1 1h-2z"/>
        </svg>
    );
}

export default function OutputDevice() {
    const [currentOutputDevice, setCurrentOutputDevice] = useState<string>("");
    const [availableOutputDevices, setAvailableOutputDevices] = useState<string[]>([]);

    const fetchOutputDevices = async () => {
        try {
            const devices = await invoke<string[]>("get_output_devices_list");
            setAvailableOutputDevices(devices);
        } catch (error) {
            console.error("Failed to fetch output devices:", error);
        }
    }

    const fetchCurrentOutputDevice = async () => {
        try {
            const device = await invoke<string>("get_selected_output_device");
            setCurrentOutputDevice(device);
        } catch (error) {
            console.error("Failed to fetch current output device:", error);
        }
    };

    const handleOutputDeviceChange = async (deviceName: string) => {
        try {
            await invoke("set_output_device", { deviceName });

            setCurrentOutputDevice(deviceName);

            setTimeout(() => {
                fetchCurrentOutputDevice();
            }, 100);
        } catch (error) {
            console.error("Failed to set output device:", error);
        }
    };

    useEffect(() => {
        fetchOutputDevices();
        fetchCurrentOutputDevice();
    }, []);

    return (
        <OptionCard
            title="Output Device"
            description="Select your audio output device"
            icon={<HeadphonesSvg />}
            currentOption={currentOutputDevice}
            availableOptions={availableOutputDevices}
            onOptionChange={handleOutputDeviceChange}
        />
    );
}