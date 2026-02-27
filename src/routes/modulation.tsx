import { createFileRoute, Outlet, useRouterState } from '@tanstack/react-router'
import ModulationPage from '@/components/layout/pages/modulation/page';

export const Route = createFileRoute('/modulation')({
  component: RouteComponent,
})

function RouteComponent() {
	const currentPath = useRouterState({
		select: (state) => state.location.pathname,
	});

	if (currentPath === '/modulation') {
		return <ModulationPage />;
	}

	return <Outlet />;
}
