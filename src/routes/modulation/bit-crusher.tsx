import { createFileRoute } from '@tanstack/react-router'
import { BitCrusherSettings } from '@/components/effect_settings/bit-crusher'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Cpu } from 'lucide-react'

export const Route = createFileRoute('/modulation/bit-crusher')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Bit Crusher" icon={Cpu} />
        <p className="text-sm text-muted-foreground">
          Reduce audio bit depth to create retro, lo-fi digital distortion.
        </p>
      </header>
      <Separator />
      <BitCrusherSettings />
    </div>
  )
}

