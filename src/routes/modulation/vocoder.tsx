import { createFileRoute } from '@tanstack/react-router'
import { VocoderSettings } from '@/components/effect_settings/vocoder'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Mic } from 'lucide-react'

export const Route = createFileRoute('/modulation/vocoder')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Vocoder" icon={Mic} />
        <p className="text-sm text-muted-foreground">
          Shape the voice through band-pass filtering for a classic robotic vocal sound.
        </p>
      </header>
      <Separator />
      <VocoderSettings />
    </div>
  )
}

