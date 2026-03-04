import { createFileRoute } from '@tanstack/react-router'
import { DistortionSettings } from '@/components/effect_settings/distortion'

export const Route = createFileRoute('/modulation/distortion')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Distortion Settings</h1>
      <DistortionSettings />
    </div>
  )
}

