import type { QueryClient } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { createRootRouteWithContext, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { DebugTrigger } from '~/components/debug-trigger'
import { Titlebar } from '~/components/layout/titlebar'
import { SplashScreen } from '~/components/splash-screen'
import { Toaster } from '~/components/ui/sonner'
import { GeneralError } from '~/features/errors/general-error'
import { NotFound } from '~/features/errors/not-found'
import { useDevToolsVisible } from '~/lib/devtools'
import { validateOrRefreshSession } from '~/shared/auth/session'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient
}>()({
  beforeLoad: async () => {
    // 1. Ensure Tauri startup state is loaded.
    const { startupState, refresh } = useStartupStore.getState()
    if (!startupState) {
      await refresh()
    }

    // 2. Validate or refresh the stored auth session.
    const session = await validateOrRefreshSession()
    const { setSession, reset, setInitialized } = useAuthStore.getState().auth
    if (session) {
      setSession(session)
    }
    else {
      reset()
    }

    // 3. Mark initialization complete — splash screen hides.
    setInitialized()
  },
  component: () => {
    const isInitializing = useAuthStore(s => s.auth.isInitializing)
    const startupState = useStartupStore(s => s.startupState)
    const isTauri = startupState !== null
    const showDebugTools = startupState?.isDebugBuild ?? false
    const devToolsVisible = useDevToolsVisible()

    if (isInitializing) {
      return <SplashScreen />
    }

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
  },
  notFoundComponent: NotFound,
  errorComponent: GeneralError,
})
