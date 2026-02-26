import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/auto-tune')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/auto-tune"!</div>
}
