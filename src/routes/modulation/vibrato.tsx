import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/vibrato')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/vibrato"!</div>
}
