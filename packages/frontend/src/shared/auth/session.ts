import type { LoginResponse } from '~/generated/types/LoginResponse'
import type { UserResponse } from '~/generated/types/UserResponse'

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
