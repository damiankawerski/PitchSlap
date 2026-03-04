import { createFileRoute } from '@tanstack/react-router'
import { ReverbSettings } from '@/components/effect_settings/reverb'

export const Route = createFileRoute('/modulation/reverb')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Reverb Settings</h1>
      <ReverbSettings />
    </div>
  )
}

