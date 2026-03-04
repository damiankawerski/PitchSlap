import { createFileRoute } from '@tanstack/react-router'
import { AutoTuneSettings } from '@/components/effect_settings/auto-tune'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Music } from 'lucide-react'

export const Route = createFileRoute('/modulation/auto-tune')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Auto-Tune" icon={Music} />
        <p className="text-sm text-muted-foreground">
          Correct pitch in real-time and snap vocals to a chosen musical scale.
        </p>
      </header>
      <Separator />
      <AutoTuneSettings />
    </div>
  )
}

