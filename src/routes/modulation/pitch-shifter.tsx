import { createFileRoute } from '@tanstack/react-router'
import { PitchShifterSettings } from '@/components/effect_settings/pitch-shifter'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { ArrowUpDown } from 'lucide-react'

export const Route = createFileRoute('/modulation/pitch-shifter')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Pitch Shifter" icon={ArrowUpDown} />
        <p className="text-sm text-muted-foreground">
          Shift the pitch of the input signal up or down by semitone steps.
        </p>
      </header>
      <Separator />
      <PitchShifterSettings />
    </div>
  )
}

