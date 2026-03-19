import { useState } from 'react'
import { authLogin } from '~/generated/client'
import { toAuthSession } from '~/shared/auth/session'
import { useAuthStore } from '~/stores/auth-store'

export interface UseLoginResult {
  login: (username: string, password: string) => Promise<void>
  isLoading: boolean
  error: string | null
}

export function useLogin(): UseLoginResult {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const login = async (username: string, password: string) => {
    setIsLoading(true)
    setError(null)

    try {
      // authLogin returns ApiResponseLoginResponse (full envelope).
      // kubb-client throws if success === false, so .data is always defined here.
      const result = await authLogin({ username, password })
      if (!result.data)
        throw new Error('Login failed')
      const session = toAuthSession(result.data)
      useAuthStore.getState().auth.setSession(session)
    }
    catch (err) {
      const message = err instanceof Error ? err.message : 'Login failed'
      setError(message)
      throw err
    }
    finally {
      setIsLoading(false)
    }
  }

  return { login, isLoading, error }
}
