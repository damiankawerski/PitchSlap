import { useEffect, useState } from 'react';

import { getAvailableVirtualDevices, getSelectedVirtualDevice } from '@/lib/invokes/config-getters';
import { setVirtualDevice } from '@/lib/invokes/config-setters';
import { CommonSettingsSelector } from '@/components/controls/selectors/common-settings-select';
import { MicVocal } from 'lucide-react';

export function VirtualDeviceSelect() {
  const [devices, setDevices] = useState<string[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
	let device = null;

  useEffect(() => {
    async function fetchDevices() {
      const availableDevices = await getAvailableVirtualDevices();
      setDevices(availableDevices);

      const currentDevice = await getSelectedVirtualDevice();
			device = currentDevice;
      setSelectedDevice(currentDevice);
    }

    fetchDevices();
  }, []);

  const handleChange = async (deviceName: string) => {
    setSelectedDevice(deviceName);
    await setVirtualDevice(deviceName);
  };

  return (
    <>
      <CommonSettingsSelector
        label="Virtual Device"
        items={devices}
        value={selectedDevice || undefined}
        onChange={handleChange}
        placeholder="Select Virtual Device"
        icon={MicVocal}
      />
    </>
  );
}
