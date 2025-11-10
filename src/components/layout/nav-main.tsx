'use client'
import { type LucideIcon } from 'lucide-react'
import { Link, useRouterState } from '@tanstack/react-router'

import {
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
} from '@/components/ui/sidebar';
import GradientText from '../GradientText';

interface NavItem {
  title: string,
  href: string,
  icon?: LucideIcon,
}

interface NavigationMainProps {
  items: NavItem[],
}

export function NavigationMain({ items }: NavigationMainProps) {
  const router = useRouterState();
  const currentPath = router.location.pathname;

  return (
    <SidebarMenu>
      {items.map((item) => {
        const isActive = currentPath === item.href;
        const Icon = item.icon;
        
        return (
          <SidebarMenuItem key={item.title}>
            <SidebarMenuButton 
              asChild 
              tooltip={item.title} 
              className='font-bold'
            >
              <Link 
                to={item.href} 
                className="flex items-center gap-2"
              >
                {Icon && (
                  <Icon className={`h-4 w-4 shrink-0 transition-colors ${
                    isActive ? 'text-secondary' : 'text-muted-foreground'
                  }`} 
                />
                )}
                <span className={isActive ? 'font-medium' : ''}>
                  {isActive ? (
                    <GradientText
                      colors={['#40ffaa', '#4079ff']}
                      animationSpeed={3}
                      showBorder={false}
                    >
                      {item.title}
                    </GradientText>
                  ) : (
                    item.title
                  )}
                </span>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        );
      })}
    </SidebarMenu>
  )
}