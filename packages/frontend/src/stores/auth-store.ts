import type { UserResponse } from '~/generated/types'
import type { AuthSession } from '~/auth/session'
import { create } from 'zustand'
import {
  clearStoredSession,
  loadStoredSession,
  persistSession,
} from '~/auth/session'

interface AuthState {
  auth: {
    user: UserResponse | null
    accessToken: string
    refreshToken: string
    isInitializing: boolean
    setSession: (session: AuthSession) => void
    clearSession: () => void
    reset: () => void
    setInitialized: () => void
  }
}

const stored = loadStoredSession()

export const useAuthStore = create<AuthState>()(set => ({
  auth: {
    user: stored?.user ?? null,
    accessToken: stored?.accessToken ?? '',
    refreshToken: stored?.refreshToken ?? '',
    isInitializing: true,

    setSession: (session: AuthSession) => {
      persistSession(session)
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
      set({
        auth: {
          ...useAuthStore.getState().auth,
          user: null,
          accessToken: '',
          refreshToken: '',
        },
      })
    },

    setInitialized: () => {
      set({
        auth: {
          ...useAuthStore.getState().auth,
          isInitializing: false,
        },
      })
    },
  },
}))
