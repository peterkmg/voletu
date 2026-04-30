import { useState } from 'react'
import { toSession } from '~/auth/session'
import { authLogin } from '~/generated/client'
import { extractErrorMessage } from '~/lib/error'
import { useAuthStore } from '~/stores/auth-store'

export interface UseSignInResult {
  signIn: (username: string, password: string) => Promise<void>
  isLoading: boolean
  error: string | null
}

export function useSignIn(): UseSignInResult {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const signIn = async (username: string, password: string) => {
    setIsLoading(true)
    setError(null)

    try {
      const result = await authLogin({ username, password })
      if (!result.data)
        throw new Error('Login failed')
      const session = toSession(result.data)
      useAuthStore.getState().login(session)
    }
    catch (err) {
      const message = extractErrorMessage(err, 'Login failed')
      setError(message)
      throw err
    }
    finally {
      setIsLoading(false)
    }
  }

  return { signIn, isLoading, error }
}
