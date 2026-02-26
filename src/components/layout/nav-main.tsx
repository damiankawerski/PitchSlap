import * as React from 'react'
import { ChevronUp, type LucideIcon } from 'lucide-react'
import { Link, useRouterState } from '@tanstack/react-router'

import {
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarMenuSub,
  SidebarMenuSubItem,
  SidebarMenuSubButton,
} from '@/components/ui/sidebar';
import GradientText from '../bits/GradientText';

interface NavItem {
  title: string,
  href?: string,
  icon?: LucideIcon,
  children?: NavItem[],
}

interface NavigationMainProps {
  items: NavItem[],
}

export function NavigationMain({ items }: NavigationMainProps) {
  const router = useRouterState();
  const currentPath = router.location.pathname;
  const [expandedItems, setExpandedItems] = React.useState<Record<string, boolean>>(() => {
    return items.reduce<Record<string, boolean>>((acc, item) => {
      if (item.children?.length) {
        acc[item.title] = item.children.some((child) => child.href === currentPath);
      }

      return acc;
    }, {});
  });

  React.useEffect(() => {
    setExpandedItems((previousState) => {
      const nextState = { ...previousState };

      items.forEach((item) => {
        if (item.children?.length && item.children.some((child) => child.href === currentPath)) {
          nextState[item.title] = true;
        }
      });

      return nextState;
    });
  }, [currentPath, items]);

  const toggleExpandedItem = (title: string) => {
    setExpandedItems((previousState) => ({
      ...previousState,
      [title]: !previousState[title],
    }));
  };

  return (
    <SidebarMenu>
      {items.map((item) => {
        const isActive = item.href ? currentPath === item.href : false;
        const hasChildren = !!item.children?.length;
        const isExpanded = !!expandedItems[item.title];
        const Icon = item.icon;
        
        return (
          <SidebarMenuItem key={item.title}>
            {hasChildren ? (
              <>
                <SidebarMenuButton
                  tooltip={item.title}
                  className="font-bold"
                  onClick={() => toggleExpandedItem(item.title)}
                >
                  {Icon && (
                    <Icon
                      className={`h-4 w-4 shrink-0 transition-colors ${
                        isExpanded ? 'text-secondary' : 'text-muted-foreground'
                      }`}
                    />
                  )}
                  <span>{item.title}</span>
                  <ChevronUp
                    className={`ml-auto h-4 w-4 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
                  />
                </SidebarMenuButton>

                {isExpanded && (
                  <SidebarMenuSub>
                    {item.children?.map((child) => {
                      const isChildActive = child.href ? currentPath === child.href : false;

                      return (
                        <SidebarMenuSubItem key={child.title}>
                          <SidebarMenuSubButton asChild isActive={isChildActive}>
                            <Link to={child.href ?? '/'}>
                              <span>{child.title}</span>
                            </Link>
                          </SidebarMenuSubButton>
                        </SidebarMenuSubItem>
                      );
                    })}
                  </SidebarMenuSub>
                )}
              </>
            ) : (
              <SidebarMenuButton
                asChild
                tooltip={item.title}
                className='font-bold'
              >
                <Link
                  to={item.href ?? '/'}
                  className="flex items-center gap-2"
                >
                  {Icon && (
                    <Icon className={`h-4 w-4 shrink-0 transition-colors ${
                      isActive ? 'text-secondary' : 'text-muted-foreground'
                    }`}
                  />
                  )}
                  <span>
                    {isActive ? (
                      <GradientText
                        colors={['#40ffaa', '#4079ff']}
                        animationSpeed={3}
                        showBorder={false}
                      >
                        <span className="font-bold">{item.title}</span>
                      </GradientText>
                    ) : (
                      item.title
                    )}
                  </span>
                </Link>
              </SidebarMenuButton>
            )}
          </SidebarMenuItem>
        );
      })}
    </SidebarMenu>
  )
}