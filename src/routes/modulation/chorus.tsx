import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/chorus')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation/chorus"!</div>
}
