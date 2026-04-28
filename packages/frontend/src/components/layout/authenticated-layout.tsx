import { Outlet, useNavigate } from '@tanstack/react-router'
import { useEffect, useState } from 'react'
import { SidebarInset, SidebarProvider } from '~/components/ui/sidebar'
import { useHealthCheck, useNodeStatus } from '~/hooks/use-node-status'
import { cn } from '~/lib/utils'
import { useAuthStore } from '~/stores/auth-store'
import { AppSidebar } from './app-sidebar'

export function AuthenticatedLayout() {
  useHealthCheck()
  useNodeStatus()

  // Reactive redirect: if auth is lost mid-session (refresh failed),
  // navigate to sign-in immediately instead of waiting for next route change.
  const status = useAuthStore(s => s.status)
  const navigate = useNavigate()
  useEffect(() => {
    if (status === 'unauthenticated') {
      void navigate({ to: '/sign-in' })
    }
  }, [status, navigate])
  const [sidebarOpen, setSidebarOpen] = useState(
    () => localStorage.getItem('voletu.sidebar') !== 'false',
  )
  return (
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
  )
}
