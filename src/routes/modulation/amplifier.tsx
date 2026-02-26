import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/amplifier')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/amplifier"!</div>
}
