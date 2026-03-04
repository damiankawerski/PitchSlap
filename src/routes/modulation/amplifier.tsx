import { createFileRoute } from '@tanstack/react-router'
import { AmplifierSettings } from '@/components/effect_settings/amplifier'

export const Route = createFileRoute('/modulation/amplifier')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Amplifier Settings</h1>
      <AmplifierSettings />
    </div>
  )
}

