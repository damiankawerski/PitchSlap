import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { InputDeviceSelect } from "./_components/input-device-select";
import { OutputDeviceSelect } from "./_components/output-device-select";
import { VirtualDeviceSelect } from "./_components/virtual-device-select";
import { LatencySelect } from "./_components/latency-select";
import { PageTitle } from "../shared/page-title";
import { Settings } from "lucide-react";
import { TransparentCard } from "@/components/ui/transparent-card";

export default function SettingsPage() {
  return (
    <div className="min-h-screen w-full overflow-y-auto">
      <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 p-6 lg:p-10">
        <header className="flex flex-col gap-2">
          <PageTitle title="Settings" icon={Settings}/>
          <p className="text-sm text-muted-foreground">
            Configure audio devices and performance options.
          </p>
        </header>

        <Separator />

        <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
          <TransparentCard title="Devices Configuration" description="Select your audio input, output, and virtual devices.">
            <div className="flex flex-col gap-4">
              <InputDeviceSelect />
              <OutputDeviceSelect />
              <VirtualDeviceSelect />
            </div>
          </TransparentCard>

          <TransparentCard title="Performance" description="Tune responsiveness vs stability.">
            <div className="flex flex-col gap-4">
              <LatencySelect />
              <Separator className="opacity-60" />
              <div className="text-sm text-muted-foreground">
                Tip: if you hear glitches, increase latency.
              </div>
            </div>
          </TransparentCard>

          <TransparentCard title="Notes" description="Important information about settings." className="lg:col-span-2">
            <div>
              <p className="mb-2">
                • Changes to device selections apply immediately; no restart needed.
              </p>
              <p className="mb-2">
                • Lower latency settings can improve responsiveness but may lead to audio glitches on some systems.
              </p>
              <p>
                • If you dont see virtual devices listed, ensure that your system has the necessary virtual audio drivers installed.
              </p>
            </div>
          </TransparentCard>
        </div>
      </div>
    </div>
  );
}