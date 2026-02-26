import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/distortion')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/distortion"!</div>
}
