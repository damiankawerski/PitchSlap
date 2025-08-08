import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function AudioControlPanel() {
  const [effects, setEffects] = useState<string[]>([]);
  const [status, setStatus] = useState<string | null>(null);

  useEffect(() => {
    invoke<string[]>("get_effects_list")
      .then(setEffects)
      .catch((err) => setStatus(`Failed to load effects: ${err}`));
  }, []);

  const applyEffect = (effect: string) => {
    invoke("set_effect", { effectName: effect })
      .then(() => setStatus(`Effect "${effect}" set.`))
      .catch((err) => setStatus(`Failed to set effect: ${err}`));
  };

  const enableMod = () => {
    invoke<string>("enable_modulation")
      .then(setStatus)
      .catch((err) => setStatus(`Failed to enable modulation: ${err}`));
  };

  const disableMod = () => {
    invoke<string>("disable_modulation")
      .then(setStatus)
      .catch((err) => setStatus(`Failed to disable modulation: ${err}`));
  };

  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-2">🎛 Audio Controls</h2>

      <div className="mb-4">
        <h3 className="font-semibold">Available Effects:</h3>
        <ul className="list-disc pl-5">
          {effects.map((effect) => (
            <li key={effect}>
              <button
                onClick={() => applyEffect(effect)}
                className="bg-blue-500 text-white px-2 py-1 m-1 rounded hover:bg-blue-700"
              >
                {effect}
              </button>
            </li>
          ))}
        </ul>
      </div>

      <div className="flex gap-2">
        <button
          onClick={enableMod}
          className="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-700"
        >
          Enable Modulation
        </button>
        <button
          onClick={disableMod}
          className="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-700"
        >
          Disable Modulation
        </button>
      </div>

      {status && (
        <div className="mt-4 text-sm text-gray-700 bg-gray-100 p-2 rounded">
          {status}
        </div>
      )}
    </div>
  );
}
