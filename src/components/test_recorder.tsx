import { useState } from "react";

import { Button } from "@/components/ui/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { startRecorderInvoke, stopRecorderInvoke } from "@/lib/invokes/recorder";

export function TestRecorder() {
	const [isRecording, setIsRecording] = useState(false);
	const [isBusy, setIsBusy] = useState(false);
	const [message, setMessage] = useState<string | null>(null);

	const handleStart = async () => {
		setIsBusy(true);
		setMessage(null);
		try {
			await startRecorderInvoke();
			setIsRecording(true);
			setMessage("Recording started");
		} catch (_error) {
			setMessage("Failed to start recording");
		} finally {
			setIsBusy(false);
		}
	};

	const handleStop = async () => {
		setIsBusy(true);
		setMessage(null);
		try {
			await stopRecorderInvoke();
			setIsRecording(false);
			setMessage("Recording stopped. Saved to recording.wav");
		} catch (_error) {
			setMessage("Failed to stop recording");
		} finally {
			setIsBusy(false);
		}
	};

	return (
		<Card className="w-full max-w-md">
			<CardHeader>
				<CardTitle>Recorder Test</CardTitle>
				<CardDescription>
					Start/stop the recorder and check the output file.
				</CardDescription>
			</CardHeader>
			<CardContent className="space-y-3">
				<div className="text-sm">
					Status: <span className="font-medium">{isRecording ? "Recording" : "Idle"}</span>
				</div>
				<div className="text-sm text-muted-foreground">
					Output: recording.wav (app working directory)
				</div>
				{message && <div className="text-sm">{message}</div>}
			</CardContent>
			<CardFooter className="gap-2">
				<Button onClick={handleStart} disabled={isBusy || isRecording}>
					Start
				</Button>
				<Button variant="secondary" onClick={handleStop} disabled={isBusy || !isRecording}>
					Stop
				</Button>
			</CardFooter>
		</Card>
	);
}
