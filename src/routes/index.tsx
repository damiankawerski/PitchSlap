import { createFileRoute } from '@tanstack/react-router'
import { AudioSpectrumVisualizer } from '@/components/visualizer/visualizer-card'

export const Route = createFileRoute('/')({
  component: Index,
})

function Index() {
  return (
    <div className="p-2">
      <AudioSpectrumVisualizer/>
    </div>
  )
}