import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/modulation"!</div>
}
