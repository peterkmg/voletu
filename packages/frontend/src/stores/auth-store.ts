import type { UserResponse } from '~/generated/types'
import type { AuthSession } from '~/shared/auth/session'
import { create } from 'zustand'
import {
  clearStoredSession,
  loadStoredSession,
  persistSession,
} from '~/shared/auth/session'

interface AuthState {
  auth: {
    user: UserResponse | null
    accessToken: string
    refreshToken: string
    setSession: (session: AuthSession) => void
    clearSession: () => void
    reset: () => void
  }
}

const stored = loadStoredSession()

export const useAuthStore = create<AuthState>()(set => ({
  auth: {
    user: stored?.user ?? null,
    accessToken: stored?.accessToken ?? '',
    refreshToken: stored?.refreshToken ?? '',

    setSession: (session: AuthSession) => {
      persistSession(session)
      localStorage.setItem('accessToken', session.accessToken)
      set({
        auth: {
          ...useAuthStore.getState().auth,
          user: session.user,
          accessToken: session.accessToken,
          refreshToken: session.refreshToken,
        },
      })
    },

    clearSession: () => {
      clearStoredSession()
      localStorage.removeItem('accessToken')
      set({
        auth: {
          ...useAuthStore.getState().auth,
          user: null,
          accessToken: '',
          refreshToken: '',
        },
      })
    },

    reset: () => {
      clearStoredSession()
      localStorage.removeItem('accessToken')
      set({
        auth: {
          ...useAuthStore.getState().auth,
          user: null,
          accessToken: '',
          refreshToken: '',
        },
      })
    },
  },
}))
