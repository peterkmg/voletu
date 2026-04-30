import { QueryCache, QueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

export const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (error, query) => {
      if (query.meta?.suppressErrorToast)
        return
      toast.error(error.message)
    },
  }),
  defaultOptions: {
    queries: {
      staleTime: 10_000,
      gcTime: 5 * 60 * 1000,
      refetchOnWindowFocus: false,
      retry: import.meta.env.DEV
        ? false
        : (failureCount, error) => {
            if (error.message.includes('401') || error.message.includes('403'))
              return false

            return failureCount < 3
          },
    },
    mutations: {
      onError: (error) => {
        toast.error(error.message)
      },
    },
  },
})
