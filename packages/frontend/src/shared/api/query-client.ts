import { QueryCache, QueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { useAuthStore } from '~/stores/auth-store'

function isUnauthorized(error: Error): boolean {
  return error.message.includes('401') || error.message.includes('Unauthorized')
}

function isForbiddenOrUnauthorized(error: Error): boolean {
  return (
    isUnauthorized(error)
    || error.message.includes('403')
    || error.message.includes('Forbidden')
  )
}

export const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (error) => {
      if (isUnauthorized(error)) {
        useAuthStore.getState().auth.reset()
        toast.error('Session expired!')
        return
      }
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
            if (isForbiddenOrUnauthorized(error as Error))
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
