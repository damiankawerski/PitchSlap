import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const LoopbackControl = () => {
  const [isRunning, setIsRunning] = useState(false);
  const [isThroughputRunning, setIsThroughputRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleLoopbackToggle = async () => {
    try {
      if (!isRunning) {
        await invoke("loopback"); 
        setIsRunning(true);
      } else {
        await invoke("stop_loopback"); 
        setIsRunning(false);
      }
      setError(null);
    } catch (err) {
      console.error("Loopback error:", err);
      setError("Wystąpił błąd przy uruchamianiu loopbacku.");
    }
  };

  const handleThroughputToggle = async () => {
    try {
      if (!isThroughputRunning) {
        await invoke("throughput"); 
        setIsThroughputRunning(true);
      } else {
        await invoke("stop_throughput"); 
        setIsThroughputRunning(false);
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
      <button onClick={handleThroughputToggle}>
        {isThroughputRunning ? "Zatrzymaj throughput" : "Uruchom throughput"}
      </button>
      {error && <p style={{ color: "red" }}>{error}</p>}
    </div>
  );
};

export default LoopbackControl;
