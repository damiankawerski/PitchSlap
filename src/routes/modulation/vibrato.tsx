import { createFileRoute } from '@tanstack/react-router'
import { VibratoSettings } from '@/components/effect_settings/vibrato'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Activity } from 'lucide-react'

export const Route = createFileRoute('/modulation/vibrato')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Vibrato" icon={Activity} />
        <p className="text-sm text-muted-foreground">
          Apply periodic pitch modulation for an expressive, wavering effect.
        </p>
      </header>
      <Separator />
      <VibratoSettings />
    </div>
  )
}

