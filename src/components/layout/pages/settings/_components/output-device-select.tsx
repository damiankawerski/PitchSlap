import { useEffect, useState } from "react";

import {
	getAvailableOutputDevices,
	getSelectedOutputDevice,
} from "@/lib/invokes/config-getters";
import { setOutputDevice } from "@/lib/invokes/config-setters";
import { CommonSettingsSelector } from "@/components/controls/selectors/common-settings-select";

export function OutputDeviceSelect() {
	const [devices, setDevices] = useState<string[]>([]);
	const [selectedDevice, setSelectedDevice] = useState<string | null>(null);

	useEffect(() => {
		async function fetchDevices() {
			const availableDevices = await getAvailableOutputDevices();
			setDevices(availableDevices);

			const currentDevice = await getSelectedOutputDevice();
			setSelectedDevice(currentDevice);
		}

		fetchDevices();
	}, []);

	const handleChange = async (deviceName: string) => {
		setSelectedDevice(deviceName);
		await setOutputDevice(deviceName);
	};

	return (
		<CommonSettingsSelector
			label="Output Device"
			items={devices}
			value={selectedDevice || undefined}
			onChange={handleChange}
			placeholder="Select Output Device"
		/>
	);
}

