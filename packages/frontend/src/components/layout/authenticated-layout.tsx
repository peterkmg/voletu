import { Outlet } from '@tanstack/react-router'
import { useState } from 'react'
import { DensityProvider } from '~/components/data-table'
import { SidebarInset, SidebarProvider } from '~/components/ui/sidebar'
import { cn } from '~/lib/utils'
import { AppSidebar } from './app-sidebar'

export function AuthenticatedLayout() {
  const [sidebarOpen, setSidebarOpen] = useState(
    () => localStorage.getItem('voletu.sidebar') !== 'false',
  )
  return (
    <DensityProvider>
      <SidebarProvider
        open={sidebarOpen}
        onOpenChange={(open) => {
          setSidebarOpen(open)
          localStorage.setItem('voletu.sidebar', String(open))
        }}
      >
        <AppSidebar />
        <SidebarInset
          className={cn(
            '@container/content',
            'has-data-[layout=fixed]:h-full',
            'peer-data-[variant=inset]:has-data-[layout=fixed]:h-full',
          )}
        >
          <Outlet />
        </SidebarInset>
      </SidebarProvider>
    </DensityProvider>
  )
}
