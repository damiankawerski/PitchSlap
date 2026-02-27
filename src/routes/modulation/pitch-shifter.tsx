import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/pitch-shifter')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/pitch-shifter"!</div>
}
