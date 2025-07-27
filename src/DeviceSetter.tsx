import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function DeviceSetter() {
  const [inputDevices, setInputDevices] = useState<string[]>([]);
  const [outputDevices, setOutputDevices] = useState<string[]>([]);
  const [virtualDevices, setVirtualDevices] = useState<string[]>([]);

  const [selectedInput, setSelectedInput] = useState<string>("");
  const [selectedOutput, setSelectedOutput] = useState<string>("");
  const [selectedVirtual, setSelectedVirtual] = useState<string>("");

  const [status, setStatus] = useState<string | null>(null);

  const fetchDevices = async () => {
    try {
      const [inputs, outputs, virtuals] = await Promise.all([
        invoke<string[]>("get_input_devices_list"),
        invoke<string[]>("get_output_devices_list"),
        invoke<string[]>("get_virtual_devices_list"),
      ]);
      setInputDevices(inputs);
      setOutputDevices(outputs);
      setVirtualDevices(virtuals);
      setSelectedInput(inputs[0] || "");
      setSelectedOutput(outputs[0] || "");
      setSelectedVirtual(virtuals[0] || "");
    } catch (err) {
      console.error("Błąd podczas pobierania urządzeń:", err);
      setStatus("❌ Nie udało się pobrać listy urządzeń");
    }
  };

  const setDevice = async (type: "input" | "output" | "virtual") => {
    let command = "";
    let name = "";

    switch (type) {
      case "input":
        command = "set_input_device";
        name = selectedInput;
        break;
      case "output":
        command = "set_output_device";
        name = selectedOutput;
        break;
      case "virtual":
        command = "set_virtual_device";
        name = selectedVirtual;
        break;
    }

    try {
      await invoke(command, { deviceName: name });
      setStatus(`✅ Ustawiono urządzenie ${type}: ${name}`);
    } catch (err) {
      console.error(`Błąd ustawiania ${type}:`, err);
      setStatus(`❌ Nie udało się ustawić urządzenia ${type}: ${name}`);
    }
  };

  useEffect(() => {
    fetchDevices();
  }, []);

  return (
    <div className="p-4 space-y-4">
      <h1 className="text-2xl font-bold">Ustawianie urządzeń audio</h1>

      {status && <div className="text-sm font-mono">{status}</div>}

      <div className="space-y-2">
        <div>
          <label className="block font-semibold">🎤 Urządzenie wejściowe</label>
          <select
            value={selectedInput}
            onChange={(e) => setSelectedInput(e.target.value)}
            className="border p-2 rounded w-full"
          >
            {inputDevices.map((device, i) => (
              <option key={`in-${i}`} value={device}>
                {device}
              </option>
            ))}
          </select>
          <button
            onClick={() => setDevice("input")}
            className="mt-2 px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700"
          >
            Ustaw wejściowe
          </button>
        </div>

        <div>
          <label className="block font-semibold">🔊 Urządzenie wyjściowe</label>
          <select
            value={selectedOutput}
            onChange={(e) => setSelectedOutput(e.target.value)}
            className="border p-2 rounded w-full"
          >
            {outputDevices.map((device, i) => (
              <option key={`out-${i}`} value={device}>
                {device}
              </option>
            ))}
          </select>
          <button
            onClick={() => setDevice("output")}
            className="mt-2 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Ustaw wyjściowe
          </button>
        </div>

        <div>
          <label className="block font-semibold">🧙 Urządzenie wirtualne</label>
          <select
            value={selectedVirtual}
            onChange={(e) => setSelectedVirtual(e.target.value)}
            className="border p-2 rounded w-full"
          >
            {virtualDevices.map((device, i) => (
              <option key={`virt-${i}`} value={device}>
                {device}
              </option>
            ))}
          </select>
          <button
            onClick={() => setDevice("virtual")}
            className="mt-2 px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-700"
          >
            Ustaw wirtualne
          </button>
        </div>
      </div>
    </div>
  );
}
