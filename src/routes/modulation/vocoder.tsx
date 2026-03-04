import { createFileRoute } from '@tanstack/react-router'
import { VocoderSettings } from '@/components/effect_settings/vocoder'

export const Route = createFileRoute('/modulation/vocoder')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <h1 className="text-2xl font-bold">Vocoder Settings</h1>
      <VocoderSettings />
    </div>
  )
}

