import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import {
  createMemoryHistory,
  createRouter,
  RouterProvider,
} from '@tanstack/react-router'
import { render } from '@testing-library/react'
import { routeTree } from '~/routeTree.gen'

function createTestQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        gcTime: 0,
      },
    },
  })
}

export function renderFileRoute(initialLocation: string) {
  const queryClient = createTestQueryClient()
  const router = createRouter({
    routeTree,
    history: createMemoryHistory({
      initialEntries: [initialLocation],
    }),
    context: { queryClient },
    defaultPreload: 'intent',
    defaultPreloadStaleTime: 0,
  })

  return {
    ...render(
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>,
    ),
    queryClient,
    router,
  }
}
