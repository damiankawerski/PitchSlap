import { createFileRoute } from '@tanstack/react-router'
import { DistortionSettings } from '@/components/effect_settings/distortion'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Zap } from 'lucide-react'

export const Route = createFileRoute('/modulation/distortion')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Distortion" icon={Zap} />
        <p className="text-sm text-muted-foreground">
          Clip and saturate the signal for a gritty, driven character.
        </p>
      </header>
      <Separator />
      <DistortionSettings />
    </div>
  )
}

