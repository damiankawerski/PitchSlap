import { createFileRoute } from '@tanstack/react-router'
import { VibratoSettings } from '@/components/effect_settings/vibrato'

export const Route = createFileRoute('/modulation/vibrato')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Vibrato Settings</h1>
      <VibratoSettings />
    </div>
  )
}

