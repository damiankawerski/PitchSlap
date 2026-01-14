import { useEffect, useRef, useState } from "react";
import { getLatency } from "@/lib/invokes/config-getters";
import { setLatency } from "@/lib/invokes/config-setters";
import { CustomInputNumber } from "@/components/controls/custom-input-number";

export function LatencySelect() {
	const [latency, setLatencyState] = useState<number | null>(null);
	const lastSavedRef = useRef<number | null>(null);

	useEffect(() => {
		async function fetchLatency() {
			const currentLatency = await getLatency();
			if (typeof currentLatency === "number") {
				lastSavedRef.current = currentLatency;
				setLatencyState(currentLatency);
			}
		}
		fetchLatency();
	}, []);

	useEffect(() => {
		if (latency === null) return;
		if (latency <= 0) return;
		if (lastSavedRef.current === latency) return;

		const handle = window.setTimeout(async () => {
			await setLatency(latency);
			lastSavedRef.current = latency;
		}, 450);

		return () => window.clearTimeout(handle);
	}, [latency]);

	return (
		<div className="flex flex-col gap-3">
			<div className="flex items-center gap-3">
				<CustomInputNumber
					placeholder="Enter latency in ms"
					value={latency ?? ""}
					min={1}
					step={1}
					label={"Latency (ms)"}
					onChange={(value) => {
						setLatencyState(value);
					}}
				/>
			</div>
		</div>
	);
}

