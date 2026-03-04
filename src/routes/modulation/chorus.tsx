import { createFileRoute } from '@tanstack/react-router'
import { ChorusSettings } from '@/components/effect_settings/chorus'
import { PageTitle } from '@/components/layout/pages/shared/page-title'
import { Separator } from '@/components/ui/separator'
import { Layers } from 'lucide-react'

export const Route = createFileRoute('/modulation/chorus')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="container mx-auto p-6 max-w-xl space-y-6">
      <header className="flex flex-col gap-2">
        <PageTitle title="Chorus" icon={Layers} />
        <p className="text-sm text-muted-foreground">
          Blend modulated copies of the signal to create a rich, layered sound.
        </p>
      </header>
      <Separator />
      <ChorusSettings />
    </div>
  )
}

