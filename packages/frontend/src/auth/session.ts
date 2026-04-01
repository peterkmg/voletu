import type { LoginResponse } from '~/generated/types/LoginResponse'
import type { UserResponse } from '~/generated/types/UserResponse'

// ---------------------------------------------------------------------------
// JWT decode (formerly ~/shared/auth/jwt-decode.ts)
// ---------------------------------------------------------------------------

/**
 * Decode a JWT's payload and extract the `exp` (expiration) claim.
 * Does NOT verify the signature — this is a client-side convenience
 * for checking expiry before making a request. The server always
 * verifies the full token via auth middleware.
 *
 * Returns the `exp` timestamp (seconds since epoch), or null if
 * the token is malformed or has no `exp` claim.
 */
export function decodeJwtExp(token: string): number | null {
  try {
    const parts = token.split('.')
    if (parts.length !== 3)
      return null

    // JWT payload is base64url-encoded. Replace URL-safe chars and decode.
    const payload = parts[1]!
      .replace(/-/g, '+')
      .replace(/_/g, '/')
    const padded = payload + '='.repeat((4 - payload.length % 4) % 4)
    const decoded = JSON.parse(atob(padded))

    return typeof decoded.exp === 'number' ? decoded.exp : null
  }
  catch {
    return null
  }
}

/**
 * Returns true if the given JWT token will expire within `thresholdSeconds`
 * from now (or is already expired).
 */
export function isTokenExpiringSoon(token: string, thresholdSeconds = 300): boolean {
  const exp = decodeJwtExp(token)
  if (exp === null)
    return true // treat unreadable tokens as expired
  return exp - Math.floor(Date.now() / 1000) < thresholdSeconds
}

// ---------------------------------------------------------------------------
// Session types & helpers (formerly ~/shared/auth/session.ts)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Refresh lock (formerly ~/shared/auth/refresh.ts)
// ---------------------------------------------------------------------------

type RefreshFn = (refreshToken: string) => Promise<AuthSession>

export interface RefreshLock {
  acquire: (refreshToken: string) => Promise<AuthSession>
}

export function createRefreshLock(refreshFn: RefreshFn): RefreshLock {
  let inflight: Promise<AuthSession> | null = null

  return {
    acquire(refreshToken: string): Promise<AuthSession> {
      if (!inflight) {
        inflight = refreshFn(refreshToken).finally(() => {
          inflight = null
        })
      }
      return inflight
    },
  }
}

/** Resolve the API base URL from the same global as kubb-client, without importing kubb-client. */
function getApiBaseUrl(): string {
  return ((globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__ ?? 'http://127.0.0.1:3000').replace(/\/+$/, '')
}

async function callRefreshEndpoint(refreshToken: string): Promise<AuthSession> {
  const response = await fetch(`${getApiBaseUrl()}/auth/refresh`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Idempotency-Key': crypto.randomUUID(),
    },
    body: JSON.stringify({ refreshToken }),
  })

  if (!response.ok) {
    throw new Error(`Refresh failed with status ${response.status}`)
  }

  const envelope = await response.json() as { success: boolean, data?: LoginResponse, error?: { message?: string } }
  if (!envelope.success || !envelope.data) {
    throw new Error(envelope.error?.message ?? 'Refresh failed')
  }

  return toAuthSession(envelope.data)
}

export const refreshLock = createRefreshLock(callRefreshEndpoint)

// ---------------------------------------------------------------------------
// Session validation (formerly bottom of ~/shared/auth/session.ts)
// ---------------------------------------------------------------------------

/**
 * Validates the stored session at app startup by verifying with the backend.
 *
 * The backend is the source of truth for token validity — client-side JWT
 * expiry checks are not sufficient (e.g., server restart with new secret,
 * DB reset, token revocation).
 *
 * Flow:
 * 1. No stored session → null (redirect to sign-in)
 * 2. No refresh token → clear stale session → null
 * 3. Refresh token exists → call backend refresh endpoint
 *    - Success → fresh tokens, persist, return session
 *    - Failure → clear stale session → null
 */
export async function validateOrRefreshSession(): Promise<AuthSession | null> {
  const session = loadStoredSession()
  if (!session)
    return null

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
