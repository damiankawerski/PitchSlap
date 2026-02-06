import { createFileRoute } from '@tanstack/react-router'
import  SoundboardTest  from '@/components/test';

export const Route = createFileRoute('/modulation')({
  component: RouteComponent,
})

function RouteComponent() {
  return <SoundboardTest />;
}
