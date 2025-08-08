import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import OptionCard from "./SettingOption/OptionCard";

const MicrophoneSvg = () => {
    return (
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 14c2.21 0 4-1.79 4-4V4c0-2.21-1.79-4-4-4S8 1.79 8 4v6c0 2.21 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
        </svg>
    );
}

export default function InputDevice() {
    const [currentInputDevice, setCurrentInputDevice] = useState<string>("");
    const [availableInputDevices, setAvailableInputDevices] = useState<string[]>([]);

    const fetchInputDevices = async () => {
        try {
            const devices = await invoke<string[]>("get_input_devices_list");
            setAvailableInputDevices(devices);
        } catch (error) {
            console.error("Failed to fetch input devices:", error);
        }
    }

    const fetchCurrentInputDevice = async () => {
        try {
            const device = await invoke<string>("get_selected_input_device");
            setCurrentInputDevice(device);
        } catch (error) {
            console.error("Failed to fetch current input device:", error);
        }
    };

    const handleInputDeviceChange = async (deviceName: string) => {
        try {
            await invoke("set_input_device", { deviceName });
            
            setCurrentInputDevice(deviceName);
            
            setTimeout(() => {
                fetchCurrentInputDevice();
            }, 100);
        } catch (error) {
            console.error("Failed to set input device:", error);
        }
    };

    useEffect(() => {
        fetchInputDevices();
        fetchCurrentInputDevice();
    }, []);

    return (
        <OptionCard
            title="Input Device"
            description="Select your audio input device"
            icon={<MicrophoneSvg />}
            currentOption={currentInputDevice}
            availableOptions={availableInputDevices}
            onOptionChange={handleInputDeviceChange}
        />
    );
}