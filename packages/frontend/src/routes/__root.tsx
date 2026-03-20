import type { QueryClient } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { createRootRouteWithContext, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { DevSeedButton } from '~/components/dev-seed-button'
import { Titlebar } from '~/components/layout/titlebar'
import { Toaster } from '~/components/ui/sonner'
import { GeneralError } from '~/features/errors/general-error'
import { NotFound } from '~/features/errors/not-found'
import { useStartupStore } from '~/stores/startup-store'

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient
}>()({
  component: () => {
    const showDebugTools = useStartupStore(s => s.startupState?.isDebugBuild ?? false)

    return (
      <div className="flex h-svh flex-col overflow-hidden">
        <Titlebar />
        <div className="flex min-h-0 flex-1 flex-col">
          <Outlet />
        </div>
        <Toaster duration={5000} />
        {showDebugTools && (
          <>
            <DevSeedButton />
            <ReactQueryDevtools buttonPosition="bottom-left" />
            <TanStackRouterDevtools position="bottom-right" />
          </>
        )}
      </div>
    )
  },
  notFoundComponent: NotFound,
  errorComponent: GeneralError,
})
