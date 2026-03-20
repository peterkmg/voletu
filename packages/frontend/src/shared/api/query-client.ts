import { QueryCache, QueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

export const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (error) => {
      toast.error(error.message)
    },
  }),
  defaultOptions: {
    queries: {
      staleTime: 10_000,
      refetchOnWindowFocus: import.meta.env.PROD,
      retry: import.meta.env.DEV
        ? false
        : (failureCount, error) => {
            // Don't retry auth errors — kubb-client already attempted refresh.
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
