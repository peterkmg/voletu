import type { QueryClient } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { createRootRouteWithContext, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { DebugTrigger } from '~/components/debug-trigger'
import { Titlebar } from '~/components/layout/titlebar'
import { Toaster } from '~/components/ui/sonner'
import { GeneralError, NotFound } from '~/features/errors'
import { useDevToolsVisible } from '~/lib/devtools'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'

function RootComponent() {
  const startupState = useStartupStore(s => s.startupState)
  const isTauri = startupState !== null
  const showDebugTools = startupState?.isDebugBuild ?? false
  const devToolsVisible = useDevToolsVisible()

  return (
    <div
      className="flex h-svh flex-col overflow-hidden"
      style={{ '--titlebar-h': isTauri ? '2rem' : '0px' } as React.CSSProperties}
    >
      {isTauri && <Titlebar />}
      <div className="flex min-h-0 flex-1 flex-col">
        <Outlet />
      </div>
      <Toaster duration={5000} />
      {showDebugTools && <DebugTrigger />}
      {showDebugTools && devToolsVisible && (
        <>
          <ReactQueryDevtools buttonPosition="bottom-left" />
          <TanStackRouterDevtools position="bottom-right" />
        </>
      )}
    </div>
  )
}

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient
}>()({
  beforeLoad: async () => {
    // 1. Ensure Tauri startup state is loaded.
    const { startupState, refresh } = useStartupStore.getState()
    if (!startupState) {
      await refresh()
    }

    // 2. Boot auth state machine (validate / refresh stored session).
    const { status, boot } = useAuthStore.getState()
    if (status === 'unknown')
      await boot()
  },
  component: RootComponent,
  notFoundComponent: NotFound,
  errorComponent: GeneralError,
})
