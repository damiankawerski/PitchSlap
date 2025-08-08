import LoopbackControl from "./LoopbackControl";
import AudioDeviceTester from "./AudioDeviceTester";
import ModulationTest from "./ModulationTest";

export default function AudioControlPanel() {
    return (
        <div>
            <h2>Audio Control Panel</h2>
            <LoopbackControl />
            <AudioDeviceTester />
            <ModulationTest />
        </div>
    )
}