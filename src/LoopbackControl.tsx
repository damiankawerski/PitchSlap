import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const LoopbackControl = () => {
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleLoopbackToggle = async () => {
    try {
      if (!isRunning) {
        await invoke("throughput"); 
        setIsRunning(true);
      } else {
        await invoke("stop_throughput"); 
        setIsRunning(false);
      }
      setError(null);
    } catch (err) {
      console.error("Loopback error:", err);
      setError("Wystąpił błąd przy uruchamianiu loopbacku.");
    }
  };

  return (
    <div>
      <h2>Loopback Control</h2>
      <button onClick={handleLoopbackToggle}>
        {isRunning ? "Zatrzymaj loopback" : "Uruchom loopback"}
      </button>
      {error && <p style={{ color: "red" }}>{error}</p>}
    </div>
  );
};

export default LoopbackControl;
