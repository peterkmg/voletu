import { Outlet } from '@tanstack/react-router'
import { DensityProvider } from '~/components/data-table'
import { SidebarInset, SidebarProvider } from '~/components/ui/sidebar'
import { cn } from '~/lib/utils'
import { AppSidebar } from './app-sidebar'

export function AuthenticatedLayout() {
  const defaultOpen = localStorage.getItem('voletu.sidebar') !== 'false'
  return (
    <DensityProvider>
      <SidebarProvider
        defaultOpen={defaultOpen}
        onOpenChange={open => localStorage.setItem('voletu.sidebar', String(open))}
      >
        <AppSidebar />
        <SidebarInset
          className={cn(
            '@container/content',
            'has-data-[layout=fixed]:h-svh',
            'peer-data-[variant=inset]:has-data-[layout=fixed]:h-[calc(100svh-(var(--spacing)*4))]',
          )}
        >
          <Outlet />
        </SidebarInset>
      </SidebarProvider>
    </DensityProvider>
  )
}
