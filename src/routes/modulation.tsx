import { createFileRoute } from '@tanstack/react-router'

import { TestRecorder } from '@/components/test_recorder'
export const Route = createFileRoute('/modulation')({
  component: RouteComponent,
})

function RouteComponent() {
	return <TestRecorder />;
}
