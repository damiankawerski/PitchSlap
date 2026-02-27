import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/recorder')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/recorder"!</div>
}
