import { AppSidebar } from '@/components/layout/app-sidebar';
import { SidebarProvider } from '@/components/ui/sidebar';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import '@/index.css';
import '@/components/bits/DarkVeil.css';
import DarkVeil from '@/components/bits/DarkVeil';

const RootLayout = () => (
  <>
    <SidebarProvider defaultOpen={true}>
      <div className="fixed inset-0 z-0 pointer-events-none">
        <DarkVeil />
      </div>
      <div className="relative z-10 flex h-svh overflow-hidden">
        <AppSidebar />
        <main className="flex-1 min-w-0 min-h-0 overflow-y-auto overscroll-none bg-transparent">
          <Outlet />
        </main>
      </div>
    </SidebarProvider>
  </>
);

export const Route = createRootRoute({ component: RootLayout });
