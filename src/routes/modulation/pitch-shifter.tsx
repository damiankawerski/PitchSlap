import { createFileRoute } from '@tanstack/react-router'
import { PitchShifterSettings } from '@/components/effect_settings/pitch-shifter'

export const Route = createFileRoute('/modulation/pitch-shifter')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Pitch Shifter Settings</h1>
      <PitchShifterSettings />
    </div>
  )
}

