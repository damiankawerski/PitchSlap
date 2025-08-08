import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import OptionCard from "./SettingOption/OptionCard";

const VirtualSvg = () => {
  return (
    <svg className="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24">
      <path d="M20 3H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h3l-1 1v1h12v-1l-1-1h3c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM4 14V5h16v9H4z" />
      <circle cx="6" cy="9" r="1" />
      <circle cx="18" cy="9" r="1" />
      <path d="M8 11h8v1H8z" />
    </svg>
  );
};

export default function VirtualDevice() {
  const [currentVirtualDevice, setCurrentVirtualDevice] = useState<string>("");
  const [availableVirtualDevices, setAvailableVirtualDevices] = useState<
    string[]
  >([]);

  const fetchVirtualDevices = async () => {
    try {
      const devices = await invoke<string[]>("get_virtual_devices_list");
      setAvailableVirtualDevices(devices);
    } catch (error) {
      console.error("Failed to fetch virtual devices:", error);
    }
  };

  const fetchCurrentVirtualDevice = async () => {
    try {
      const device = await invoke<string>("get_selected_virtual_input");
      setCurrentVirtualDevice(device);
    } catch (error) {
      console.error("Failed to fetch current virtual device:", error);
    }
  };

  const handleVirtualDeviceChange = async (deviceName: string) => {
    try {
      await invoke("set_virtual_device", { deviceName });

      setCurrentVirtualDevice(deviceName);

      setTimeout(() => {
        fetchCurrentVirtualDevice();
      }, 100);
    } catch (error) {
      console.error("Failed to set virtual device:", error);
    }
  };

  useEffect(() => {
    fetchVirtualDevices();
    fetchCurrentVirtualDevice();
  }, []);

  return (
    <OptionCard
      title="Virtual Input"
      description="Select your virtual input device"
      icon={<VirtualSvg />}
      currentOption={currentVirtualDevice}
      availableOptions={availableVirtualDevices}
      onOptionChange={handleVirtualDeviceChange}
    />
  );
}
