import "./App.css";
import DeviceSetter from "./DeviceSetter";
import LoopbackControl from "./LoopbackControl";
import AudioDeviceTester from "./AudioDeviceTester";

function App() {
  return (
    <div>
      <h1>Audio Loopback</h1>
      <LoopbackControl />
      {/* <DeviceSetter /> */}
      <AudioDeviceTester />
    </div>
  );
}

export default App;
