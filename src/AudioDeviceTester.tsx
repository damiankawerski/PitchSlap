import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const AudioDeviceTester: React.FC = () => {
  const [inputDevices, setInputDevices] = useState<string[]>([]);
  const [outputDevices, setOutputDevices] = useState<string[]>([]);
  const [virtualDevices, setVirtualDevices] = useState<string[]>([]);

  const [selectedInput, setSelectedInput] = useState<string>("");
  const [selectedOutput, setSelectedOutput] = useState<string>("");
  const [selectedVirtual, setSelectedVirtual] = useState<string>("");
  const [latency, setLatency] = useState<number>(0);

  const [newLatency, setNewLatency] = useState<string>("");

  const fetchDevices = async () => {
    setInputDevices(await invoke<string[]>("get_input_devices_list"));
    setOutputDevices(await invoke<string[]>("get_output_devices_list"));
    setVirtualDevices(await invoke<string[]>("get_virtual_devices_list"));
  };

  const fetchSettings = async () => {
    setSelectedInput(await invoke<string>("get_selected_input_device"));
    setSelectedOutput(await invoke<string>("get_selected_output_device"));
    setSelectedVirtual(await invoke<string>("get_selected_virtual_input"));
    setLatency(await invoke<number>("get_latency"));
  };

  const setDevice = async (type: "input" | "output" | "virtual", name: string) => {
    const commandMap = {
      input: "set_input_device",
      output: "set_output_device",
      virtual: "set_virtual_device",
    };
    try {
      await invoke(commandMap[type], { deviceName: name });
      await fetchSettings();
    } catch (err) {
      console.error(`Error setting ${type} device:`, err);
    }
  };

  const updateLatency = async () => {
    try {
      await invoke("set_latency", { latency: parseFloat(newLatency) });
      await fetchSettings();
      setNewLatency("");
    } catch (err) {
      console.error("Failed to set latency:", err);
    }
  };

  useEffect(() => {
    fetchDevices();
    fetchSettings();
  }, []);

  return (
    <div style={{ fontFamily: "sans-serif", padding: 16 }}>
      <h2>🎧 Audio Device Tester</h2>

      <div>
        <h3>Input Devices</h3>
        <ul>
          {inputDevices.map((dev) => (
            <li key={dev}>
              <button onClick={() => setDevice("input", dev)}>
                {dev} {dev === selectedInput && "(selected)"}
              </button>
            </li>
          ))}
        </ul>
      </div>

      <div>
        <h3>Output Devices</h3>
        <ul>
          {outputDevices.map((dev) => (
            <li key={dev}>
              <button onClick={() => setDevice("output", dev)}>
                {dev} {dev === selectedOutput && "(selected)"}
              </button>
            </li>
          ))}
        </ul>
      </div>

      <div>
        <h3>Virtual Input Devices</h3>
        <ul>
          {virtualDevices.map((dev) => (
            <li key={dev}>
              <button onClick={() => setDevice("virtual", dev)}>
                {dev} {dev === selectedVirtual && "(selected)"}
              </button>
            </li>
          ))}
        </ul>
      </div>

      <div>
        <h3>Latency</h3>
        <p>Current latency: {latency} ms</p>
        <input
          type="number"
          value={newLatency}
          onChange={(e) => setNewLatency(e.target.value)}
          placeholder="Enter new latency"
        />
        <button onClick={updateLatency}>Set latency</button>
      </div>
    </div>
  );
};

export default AudioDeviceTester;
