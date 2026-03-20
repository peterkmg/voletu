import type { LoginResponse } from '~/generated/types/LoginResponse'
import type { UserResponse } from '~/generated/types/UserResponse'
import { isTokenExpiringSoon } from './jwt-decode'
import { refreshLock } from './refresh'

const STORAGE_KEY = 'voletu.auth.session'

export interface AuthSession {
  accessToken: string
  refreshToken: string
  user: UserResponse
}

export function toAuthSession(payload: LoginResponse): AuthSession {
  return {
    accessToken: payload.accessToken,
    refreshToken: payload.refreshToken,
    user: payload.user,
  }
}

export function loadStoredSession(): AuthSession | null {
  const raw = globalThis.localStorage.getItem(STORAGE_KEY)
  if (!raw) {
    return null
  }

  try {
    return JSON.parse(raw) as AuthSession
  }
  catch {
    globalThis.localStorage.removeItem(STORAGE_KEY)
    return null
  }
}

export function persistSession(session: AuthSession): void {
  globalThis.localStorage.setItem(STORAGE_KEY, JSON.stringify(session))
}

export function clearStoredSession(): void {
  globalThis.localStorage.removeItem(STORAGE_KEY)
}

/**
 * Validates the stored session at app startup.
 *
 * - If no stored session exists, returns null.
 * - If the access token is still valid (exp in the future), returns the session as-is.
 * - If the access token is expired or expiring soon, attempts a silent refresh
 *   using the stored refresh token.
 *   - On success: persists the new session and returns it.
 *   - On failure: clears the stored session and returns null.
 *
 * The refresh token is opaque (not a JWT), so we always let the server
 * decide whether it is still valid.
 */
export async function validateOrRefreshSession(): Promise<AuthSession | null> {
  const session = loadStoredSession()
  if (!session) return null

  // Access token still valid — no refresh needed.
  // Use threshold=0 here: at startup we only care about actual expiry,
  // not the 5-min proactive window (that's handled per-request in kubb-client).
  if (!isTokenExpiringSoon(session.accessToken, 0)) {
    return session
  }

  // Access token expired — attempt refresh.
  if (!session.refreshToken) {
    clearStoredSession()
    return null
  }

  try {
    const newSession = await refreshLock.acquire(session.refreshToken)
    persistSession(newSession)
    return newSession
  }
  catch {
    clearStoredSession()
    return null
  }
}
