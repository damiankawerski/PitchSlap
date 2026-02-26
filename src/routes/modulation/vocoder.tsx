import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/vocoder')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/vocoder"!</div>
}
