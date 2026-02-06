import React, { useState } from "react";
import { invoke } from '@tauri-apps/api/core';

type ActionState = {
	status: "idle" | "working" | "success" | "error";
	message: string;
};

const initialState: ActionState = { status: "idle", message: "" };

export default function SoundboardTest() {
	const [listening, setListening] = useState(false);
	const [uploadPath, setUploadPath] = useState("");
	const [deleteName, setDeleteName] = useState("");
	const [lastResult, setLastResult] = useState<ActionState>(initialState);

	const setWorking = (message: string) =>
		setLastResult({ status: "working", message });

	const setError = (message: string) =>
		setLastResult({ status: "error", message });

	const setSuccess = (message: string) =>
		setLastResult({ status: "success", message });

	const startListening = async () => {
		try {
			setWorking("Starting listener...");
			await invoke("soundboard_start_listening");
			setListening(true);
			setSuccess("Listener started.");
		} catch (err) {
			setError(String(err));
		}
	};

	const stopListening = async () => {
		try {
			setWorking("Stopping listener...");
			await invoke("soundboard_stop_listening");
			setListening(false);
			setSuccess("Listener stopped.");
		} catch (err) {
			setError(String(err));
		}
	};

	const uploadFile = async () => {
		if (!uploadPath.trim()) {
			setError("Provide a source path.");
			return;
		}
		try {
			setWorking("Uploading file...");
			const target = (await invoke("soundboard_upload_file", {
				sourcePath: uploadPath,
			})) as string;
			setSuccess(`Uploaded to: ${target}`);
		} catch (err) {
			setError(String(err));
		}
	};

	const deleteFile = async () => {
		if (!deleteName.trim()) {
			setError("Provide a file name.");
			return;
		}
		try {
			setWorking("Deleting file...");
			await invoke("soundboard_delete_file", { fileName: deleteName });
			setSuccess("File deleted.");
		} catch (err) {
			setError(String(err));
		}
	};

	return (
		<div className="sb-wrap">
			<style>{`
				@import url("https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;600;700&display=swap");

				:root {
					--sb-ink: #101216;
					--sb-ink-soft: #3b4252;
					--sb-accent: #ff6b35;
					--sb-accent-2: #2ec4b6;
					--sb-bg-1: #f7f2ea;
					--sb-bg-2: #f3e6ff;
					--sb-card: rgba(255, 255, 255, 0.72);
					--sb-border: rgba(16, 18, 22, 0.1);
				}

				.sb-wrap {
					font-family: "Space Grotesk", sans-serif;
					color: var(--sb-ink);
					padding: 28px;
					background: radial-gradient(1200px 800px at 10% 0%, var(--sb-bg-2), transparent),
						radial-gradient(900px 700px at 90% 10%, #e0f7f4, transparent),
						linear-gradient(160deg, var(--sb-bg-1), #fff);
					min-height: 100vh;
					display: grid;
					gap: 18px;
				}

				.sb-header {
					display: flex;
					align-items: center;
					justify-content: space-between;
				}

				.sb-title {
					font-size: 28px;
					font-weight: 700;
					letter-spacing: -0.02em;
				}

				.sb-status {
					font-size: 13px;
					padding: 6px 10px;
					border-radius: 999px;
					background: #fff;
					border: 1px solid var(--sb-border);
					color: var(--sb-ink-soft);
				}

				.sb-grid {
					display: grid;
					grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
					gap: 16px;
				}

				.sb-card {
					background: var(--sb-card);
					border: 1px solid var(--sb-border);
					border-radius: 18px;
					padding: 16px;
					box-shadow: 0 10px 30px rgba(0, 0, 0, 0.06);
					backdrop-filter: blur(8px);
					animation: rise 420ms ease-out;
				}

				.sb-card h3 {
					margin: 0 0 10px 0;
					font-size: 16px;
				}

				.sb-field {
					display: grid;
					gap: 8px;
					margin-bottom: 12px;
				}

				.sb-input {
					border: 1px solid var(--sb-border);
					border-radius: 12px;
					padding: 10px 12px;
					font-size: 14px;
					background: #fff;
					outline: none;
				}

				.sb-actions {
					display: flex;
					flex-wrap: wrap;
					gap: 8px;
				}

				.sb-btn {
					border: 1px solid var(--sb-border);
					background: #fff;
					color: var(--sb-ink);
					padding: 9px 14px;
					border-radius: 12px;
					font-weight: 600;
					cursor: pointer;
					transition: transform 120ms ease, box-shadow 120ms ease;
				}

				.sb-btn:hover {
					transform: translateY(-1px);
					box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08);
				}

				.sb-btn--accent {
					background: var(--sb-accent);
					color: #fff;
					border-color: transparent;
				}

				.sb-btn--teal {
					background: var(--sb-accent-2);
					color: #fff;
					border-color: transparent;
				}

				.sb-result {
					padding: 10px 12px;
					border-radius: 12px;
					background: #fff;
					border: 1px solid var(--sb-border);
					font-size: 13px;
					color: var(--sb-ink-soft);
					min-height: 40px;
				}

				.sb-result.success {
					border-color: rgba(46, 196, 182, 0.45);
					color: #136d65;
				}

				.sb-result.error {
					border-color: rgba(255, 107, 53, 0.5);
					color: #8a2b10;
				}

				@keyframes rise {
					from {
						transform: translateY(8px);
						opacity: 0;
					}
					to {
						transform: translateY(0);
						opacity: 1;
					}
				}
			`}</style>

			<div className="sb-header">
				<div className="sb-title">Soundboard Test Rig</div>
				<div className="sb-status">
					{listening ? "Listening: ON" : "Listening: OFF"}
				</div>
			</div>

			<div className="sb-grid">
				<div className="sb-card">
					<h3>Listener Control</h3>
					<div className="sb-actions">
						<button className="sb-btn sb-btn--teal" onClick={startListening}>
							Start Listening
						</button>
						<button className="sb-btn" onClick={stopListening}>
							Stop Listening
						</button>
					</div>
				</div>

				<div className="sb-card">
					<h3>Upload Audio File</h3>
					<div className="sb-field">
						<input
							className="sb-input"
							placeholder="C:\\path\\to\\file.wav"
							value={uploadPath}
							onChange={(event) => setUploadPath(event.target.value)}
						/>
					</div>
					<div className="sb-actions">
						<button className="sb-btn sb-btn--accent" onClick={uploadFile}>
							Upload
						</button>
					</div>
				</div>

				<div className="sb-card">
					<h3>Delete Audio File</h3>
					<div className="sb-field">
						<input
							className="sb-input"
							placeholder="file.wav"
							value={deleteName}
							onChange={(event) => setDeleteName(event.target.value)}
						/>
					</div>
					<div className="sb-actions">
						<button className="sb-btn" onClick={deleteFile}>
							Delete
						</button>
					</div>
				</div>
			</div>

			<div
				className={`sb-result ${
					lastResult.status === "success"
						? "success"
						: lastResult.status === "error"
						? "error"
						: ""
				}`}
			>
				{lastResult.status === "idle" && "Waiting for action..."}
				{lastResult.status === "working" && lastResult.message}
				{lastResult.status === "success" && lastResult.message}
				{lastResult.status === "error" && lastResult.message}
			</div>
		</div>
	);
}
