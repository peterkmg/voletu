import type { QueryClient } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { createRootRouteWithContext, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { DebugTrigger } from '~/components/debug-trigger'
import { Titlebar } from '~/components/layout/titlebar'
import { Toaster } from '~/components/ui/sonner'
import { useDevToolsVisible } from '~/lib/devtools'
import { ensureBootstrapped } from '~/platform/runtime/bootstrap'
import { useStartupStore } from '~/stores/startup-store'
import { GeneralError, NotFound } from '~/views/errors'

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
    await ensureBootstrapped()
  },
  component: RootComponent,
  notFoundComponent: NotFound,
  errorComponent: GeneralError,
})
