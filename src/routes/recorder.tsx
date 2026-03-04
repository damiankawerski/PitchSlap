import { createFileRoute } from '@tanstack/react-router'
import RecorderPage from '@/components/layout/pages/recorder/page'

export const Route = createFileRoute('/recorder')({
  component: RouteComponent,
})

function RouteComponent() {
  return <RecorderPage />
}
