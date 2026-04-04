import { QueryClientProvider } from '@tanstack/react-query'
import { createRouter, RouterProvider } from '@tanstack/react-router'
import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { setApiBaseUrl } from '~/api/client'
import { queryClient } from '~/api/query-client'
import { ThemeProvider } from '~/context/theme-provider'
import { useStartupStore } from '~/stores/startup-store'
import { routeTree } from './routeTree.gen'

import '~/i18n/config'
import '~/styles/index.css'

const router = createRouter({
  routeTree,
  context: { queryClient },
  defaultPreload: 'intent',
  defaultPreloadStaleTime: 0,
})

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

async function init() {
  try {
    await useStartupStore.getState().refresh()
    const { startupState } = useStartupStore.getState()
    if (startupState?.apiBaseUrl) {
      setApiBaseUrl(startupState.apiBaseUrl)
    }
  }
  catch {
    // Tauri may not be available in web mode — continue without startup state
  }

  const rootElement = document.getElementById('root')!
  if (!rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement)
    root.render(
      <StrictMode>
        <QueryClientProvider client={queryClient}>
          <ThemeProvider>
            <RouterProvider router={router} />
          </ThemeProvider>
        </QueryClientProvider>
      </StrictMode>,
    )
  }
}

init()
