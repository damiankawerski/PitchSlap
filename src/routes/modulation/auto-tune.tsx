import { createFileRoute } from '@tanstack/react-router'
import { AutoTuneSettings } from '@/components/effect_settings/auto-tune'

export const Route = createFileRoute('/modulation/auto-tune')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Auto-Tune Settings</h1>
      <AutoTuneSettings />
    </div>
  )
}

