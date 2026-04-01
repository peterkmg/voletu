import type { UserResponse } from '~/generated/types'
import type { AuthSession } from '~/auth/session'
import { create } from 'zustand'
import {
  clearSession,
  loadSession,
  refreshTokens,
  saveSession,
  verifyToken,
} from '~/auth/session'

// ---------------------------------------------------------------------------
// Auth state machine
// ---------------------------------------------------------------------------
//
// States:
//   unknown         → app just started, tokens may exist in storage
//   validating      → verifying access token with backend (GET /auth/me)
//   valid           → tokens confirmed working, app is usable
//   refreshing      → exchanging refresh token for new access token
//   unauthenticated → no valid session, redirect to sign-in
//
// Transitions:
//   unknown → validating → valid          (token verified)
//   unknown → validating → refreshing → valid  (token expired, refresh ok)
//   unknown → validating → refreshing → unauthenticated (refresh failed)
//   unknown → unauthenticated             (no stored tokens)
//   valid → refreshing → valid            (proactive or 401-triggered refresh)
//   valid → refreshing → unauthenticated  (refresh failed)
//   * → unauthenticated                   (logout)
//
// ---------------------------------------------------------------------------

export type AuthStatus = 'unknown' | 'validating' | 'valid' | 'refreshing' | 'unauthenticated'

interface AuthStore {
  status: AuthStatus
  accessToken: string | null
  refreshToken: string | null
  user: UserResponse | null

  /** One-time startup: load stored session → verify → valid or unauthenticated */
  boot: () => Promise<void>
  /** Called on 401 or near-expiry: refresh tokens. Returns true if caller should replay. */
  onUnauthorized: () => Promise<boolean>
  /** Called after successful sign-in */
  login: (session: AuthSession) => void
  /** Clear session and redirect to sign-in */
  logout: () => void
}

// Dedup concurrent refresh calls (multiple 401s from parallel requests)
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

    // Try to verify the access token with the backend
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
    catch {
      // Access token invalid — try refresh
    }

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
    if (inflight) return inflight

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
