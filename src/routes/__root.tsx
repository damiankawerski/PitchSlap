import { AppSidebar } from '@/components/layout/app-sidebar';
import { SidebarProvider } from '@/components/ui/sidebar';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import '@/index.css';
import Iridescence from '@/components/Iridescence';

const RootLayout = () => (
  <>
    <SidebarProvider
    defaultOpen={true}
    >
      <div className="fixed inset-0 -z-10">
        <Iridescence
          color={[0.22, 0.0, 0.11]} 
          mouseReact={true}
          amplitude={0.25}
          speed={1.2}
        />
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


