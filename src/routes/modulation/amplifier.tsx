import { createFileRoute } from '@tanstack/react-router'
import { AmplifierSettings } from '@/components/effect_settings/amplifier'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Volume2 } from 'lucide-react'

export const Route = createFileRoute('/modulation/amplifier')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Amplifier" icon={Volume2} />
        <p className="text-sm text-muted-foreground">
          Boost or reduce the overall signal volume with adjustable gain.
        </p>
      </header>
      <Separator />
      <AmplifierSettings />
    </div>
  )
}

