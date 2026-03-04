import { createFileRoute } from '@tanstack/react-router'
import { ReverbSettings } from '@/components/effect_settings/reverb'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Waves } from 'lucide-react'

export const Route = createFileRoute('/modulation/reverb')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Reverb" icon={Waves} />
        <p className="text-sm text-muted-foreground">
          Simulate acoustic spaces with adjustable room size, damping and wet/dry mix.
        </p>
      </header>
      <Separator />
      <ReverbSettings />
    </div>
  )
}

