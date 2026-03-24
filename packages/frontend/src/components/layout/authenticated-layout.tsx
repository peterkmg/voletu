import { Outlet } from '@tanstack/react-router'
import { useState } from 'react'
import { DensityProvider } from '~/components/data-table'
import { SidebarInset, SidebarProvider } from '~/components/ui/sidebar'
import { cn } from '~/lib/utils'
import { useHealthCheck } from '~/shared/api/hooks/use-node-status'
import { AppSidebar } from './app-sidebar'

export function AuthenticatedLayout() {
  useHealthCheck()
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
            'peer-data-[variant=inset]:has-data-[layout=fixed]:h-[calc(100%-1rem)]',
          )}
        >
          <Outlet />
        </SidebarInset>
      </SidebarProvider>
    </DensityProvider>
  )
}
