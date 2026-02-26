import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/modulation/reverb')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div className="text-white">TEST REVERB</div>
}
