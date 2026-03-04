import { createFileRoute } from '@tanstack/react-router'
import { ChorusSettings } from '@/components/effect_settings/chorus'

export const Route = createFileRoute('/modulation/chorus')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Chorus Settings</h1>
      <ChorusSettings />
    </div>
  )
}

