import {
  SidebarContent,
  SidebarFooter,
  SidebarMenu,
  SidebarMenuItem,
  SidebarHeader,
  Sidebar,
} from '@/components/ui/sidebar';

import { Home, Settings, Sparkles } from 'lucide-react';
import { NAVIGATION } from '@/lib/consts/navigation';
import { NavigationMain } from './nav-main';
import GradientText from '../bits/GradientText';
import { QuickActions } from './quick-actions';

export function AppSidebar() {
  const nav_items = [
    { title: 'Home', href: NAVIGATION.HOME, icon: Home },
    { title: 'Modulation', href: NAVIGATION.MODULATION, icon: Sparkles },
    { title: 'Settings', href: NAVIGATION.SETTINGS, icon: Settings },
  ];

  return (
    <>
      <Sidebar
        className='border-r-accent-foreground'
      >
        <SidebarHeader className="border-b border-sidebar-border mb-4">
          <SidebarMenu>
            <SidebarMenuItem>
              <GradientText
                colors={['#40ffaa', '#4079ff', '#40ffaa', '#4079ff', '#40ffaa']}
                animationSpeed={3}
                showBorder={false}
              >
                <h1 className="font-bold text-3xl flex justify-start items-start">PitchSlap</h1>
              </GradientText>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>

        <SidebarContent>
          <NavigationMain items={nav_items} />
        </SidebarContent>

        <SidebarFooter className="border-t border-sidebar-border flex items-center">
          <QuickActions></QuickActions>
        </SidebarFooter>

        <SidebarFooter className="border-t border-sidebar-border">
          <div className="flex items-center justify-center space-x-1 lg:space-x-2">
            <div className="w-1 h-1 lg:w-2 lg:h-2 bg-purple-500 rounded-full animate-pulse"></div>
            <div
              className="w-1 h-1 lg:w-2 lg:h-2 bg-pink-500 rounded-full animate-pulse"
              style={{ animationDelay: '0.5s' }}
            ></div>
            <div
              className="w-1 h-1 lg:w-2 lg:h-2 bg-orange-500 rounded-full animate-pulse"
              style={{ animationDelay: '1s' }}
            ></div>
          </div>
        </SidebarFooter>
      </Sidebar>
    </>
  );
}
