import type { LoginResponse } from '~/generated/types/LoginResponse'
import type { UserResponse } from '~/generated/types/UserResponse'
import { getApiBaseUrl } from '~/platform/runtime/api-base-url'

export function decodeJwtExp(token: string): number | null {
  try {
    const parts = token.split('.')

    if (parts.length !== 3)
      return null

    const payload = parts[1]!.replace(/-/g, '+').replace(/_/g, '/')
    const padded = payload + '='.repeat((4 - payload.length % 4) % 4)
    const decoded = JSON.parse(atob(padded))

    return typeof decoded.exp === 'number' ? decoded.exp : null
  }
  catch {
    return null
  }
}

export function isTokenExpiringSoon(token: string, thresholdSeconds = 300): boolean {
  const exp = decodeJwtExp(token)

  if (exp === null)
    return true

  return exp - Math.floor(Date.now() / 1000) < thresholdSeconds
}

const STORAGE_KEY = 'voletu.auth.session'

export interface AuthSession {
  accessToken: string
  refreshToken: string
  user: UserResponse
}

export function toSession(payload: LoginResponse): AuthSession {
  return {
    accessToken: payload.accessToken,
    refreshToken: payload.refreshToken,
    user: payload.user,
  }
}

export function loadSession(): AuthSession | null {
  const raw = globalThis.localStorage.getItem(STORAGE_KEY)

  if (!raw)
    return null

  try {
    return JSON.parse(raw) as AuthSession
  }
  catch {
    globalThis.localStorage.removeItem(STORAGE_KEY)
    return null
  }
}

export function saveSession(session: AuthSession): void {
  globalThis.localStorage.setItem(STORAGE_KEY, JSON.stringify(session))
}

export function clearSession(): void {
  globalThis.localStorage.removeItem(STORAGE_KEY)
}

export async function verifyToken(accessToken: string): Promise<UserResponse> {
  const response = await fetch(`${getApiBaseUrl()}/auth/me`, {
    headers: { Authorization: `Bearer ${accessToken}` },
  })

  if (!response.ok) {
    throw new Error(`Token verification failed: ${response.status}`)
  }

  const envelope = await response.json() as { success: boolean, data?: UserResponse }
  if (!envelope.success || !envelope.data) {
    throw new Error('Token verification failed')
  }

  return envelope.data
}

export async function refreshTokens(refreshToken: string): Promise<AuthSession> {
  const response = await fetch(`${getApiBaseUrl()}/auth/refresh`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Idempotency-Key': crypto.randomUUID(),
    },
    body: JSON.stringify({ refreshToken }),
  })

  if (!response.ok) {
    throw new Error(`Refresh failed: ${response.status}`)
  }

  const envelope = await response.json() as { success: boolean, data?: LoginResponse, error?: { message?: string } }
  if (!envelope.success || !envelope.data) {
    throw new Error(envelope.error?.message ?? 'Refresh failed')
  }

  return toSession(envelope.data)
}
