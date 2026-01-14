import { setInputDevice } from "@/lib/invokes/config-setters";
import { getSelectedInputDevice, getAvailableInputDevices } from "@/lib/invokes/config-getters";
import { useEffect, useState } from "react";
import { CommonSettingsSelector } from "@/components/controls/selectors/common-settings-select";
import { Mic } from "lucide-react";
export function InputDeviceSelect() {

  const [devices, setDevices] = useState<string[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);

  useEffect(() => {
    async function fetchDevices() {
      const availableDevices = await getAvailableInputDevices();
      setDevices(availableDevices);
      const currentDevice = await getSelectedInputDevice();
      setSelectedDevice(currentDevice);
    }
    fetchDevices();
  }, []);

  const handleChange = async (deviceName: string) => {
    setSelectedDevice(deviceName);
    await setInputDevice(deviceName);
  };

  return (
    <CommonSettingsSelector
      label="Input Device"
      items={devices}
      value={selectedDevice || undefined}
      onChange={handleChange}
      placeholder="Select Input Device"
      icon={Mic}
    />
  );
}