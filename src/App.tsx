import "./App.css";
import DeviceSetter from "./DeviceSetter";
import LoopbackControl from "./LoopbackControl";

function App() {
  return (
    <div>
      <h1>Audio Loopback</h1>
      <LoopbackControl />
      <DeviceSetter />
    </div>
  );
}

export default App;
