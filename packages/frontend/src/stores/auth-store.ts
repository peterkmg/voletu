import type { AuthSession } from '~/auth/session'
import type { UserResponse } from '~/generated/types'
import { create } from 'zustand'
import {
  clearSession,
  loadSession,
  refreshTokens,
  saveSession,
  verifyToken,
} from '~/auth/session'

export type AuthStatus = 'unknown' | 'validating' | 'valid' | 'refreshing' | 'unauthenticated'

interface AuthStore {
  status: AuthStatus
  accessToken: string | null
  refreshToken: string | null
  user: UserResponse | null
  boot: () => Promise<void>
  onUnauthorized: () => Promise<boolean>
  login: (session: AuthSession) => void
  logout: () => void
}

let inflight: Promise<boolean> | null = null

export const useAuthStore = create<AuthStore>()((set, get) => ({
  status: 'unknown',
  accessToken: null,
  refreshToken: null,
  user: null,

  boot: async () => {
    const stored = loadSession()
    if (!stored?.accessToken) {
      clearSession()
      set({ status: 'unauthenticated' })
      return
    }

    set({
      status: 'validating',
      accessToken: stored.accessToken,
      refreshToken: stored.refreshToken,
      user: stored.user,
    })

    try {
      const user = await verifyToken(stored.accessToken)
      set({ status: 'valid', user })
      return
    }
    catch { }

    if (!stored.refreshToken) {
      clearSession()
      set({ status: 'unauthenticated', accessToken: null, refreshToken: null, user: null })
      return
    }

    set({ status: 'refreshing' })
    try {
      const session = await refreshTokens(stored.refreshToken)
      saveSession(session)
      set({
        status: 'valid',
        accessToken: session.accessToken,
        refreshToken: session.refreshToken,
        user: session.user,
      })
    }
    catch {
      clearSession()
      set({ status: 'unauthenticated', accessToken: null, refreshToken: null, user: null })
    }
  },

  onUnauthorized: () => {
    if (inflight)
      return inflight

    inflight = (async () => {
      const { refreshToken: token } = get()
      if (!token) {
        clearSession()
        set({ status: 'unauthenticated', accessToken: null, refreshToken: null, user: null })
        return false
      }

      set({ status: 'refreshing' })
      try {
        const session = await refreshTokens(token)
        saveSession(session)
        set({
          status: 'valid',
          accessToken: session.accessToken,
          refreshToken: session.refreshToken,
          user: session.user,
        })
        return true
      }
      catch {
        clearSession()
        set({ status: 'unauthenticated', accessToken: null, refreshToken: null, user: null })
        return false
      }
    })().finally(() => {
      inflight = null
    })

    return inflight
  },

  login: (session: AuthSession) => {
    saveSession(session)
    set({
      status: 'valid',
      accessToken: session.accessToken,
      refreshToken: session.refreshToken,
      user: session.user,
    })
  },

  logout: () => {
    clearSession()
    set({ status: 'unauthenticated', accessToken: null, refreshToken: null, user: null })
  },
}))
