import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/bit-crusher')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/bit-crusher"!</div>
}
