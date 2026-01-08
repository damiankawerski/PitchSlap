import { AppSidebar } from '@/components/layout/app-sidebar';
import { SidebarProvider } from '@/components/ui/sidebar';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import '@/index.css';
import '@/components/bits/DarkVeil.css';
import DarkVeil from '@/components/bits/DarkVeil';

const RootLayout = () => (
  <>
    <SidebarProvider defaultOpen={true}>
      <div className="fixed inset-0 -z-10 pointer-events-none">
        <DarkVeil />
      </div>
      <div className="relative z-0 flex min-h-screen">
        <AppSidebar />
        <main className="flex-1">
          <Outlet />
        </main>
      </div>
    </SidebarProvider>
  </>
);

export const Route = createRootRoute({ component: RootLayout });
