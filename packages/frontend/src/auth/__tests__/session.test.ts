import { beforeEach, describe, expect, it } from 'vitest'
import {
  clearSession,
  decodeJwtExp,
  isTokenExpiringSoon,
  loadSession,
  saveSession,
  toSession,
} from '../session'

// ---------------------------------------------------------------------------
// JWT decode tests
// ---------------------------------------------------------------------------

function makeJwt(payload: Record<string, unknown>): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '')
  const body = btoa(JSON.stringify(payload))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '')
  return `${header}.${body}.fake-signature`
}

describe('decodeJwtExp', () => {
  it('returns exp from a valid JWT', () => {
    const exp = Math.floor(Date.now() / 1000) + 3600
    expect(decodeJwtExp(makeJwt({ sub: 'user', exp }))).toBe(exp)
  })

  it('returns null for a JWT without exp', () => {
    expect(decodeJwtExp(makeJwt({ sub: 'user' }))).toBeNull()
  })

  it('returns null for malformed token', () => {
    expect(decodeJwtExp('not-a-jwt')).toBeNull()
  })

  it('returns null for empty string', () => {
    expect(decodeJwtExp('')).toBeNull()
  })
})

describe('isTokenExpiringSoon', () => {
  it('returns false for a token expiring in 1 hour', () => {
    const exp = Math.floor(Date.now() / 1000) + 3600
    expect(isTokenExpiringSoon(makeJwt({ exp }), 300)).toBe(false)
  })

  it('returns true for a token expiring in 2 minutes (below 5-min threshold)', () => {
    const exp = Math.floor(Date.now() / 1000) + 120
    expect(isTokenExpiringSoon(makeJwt({ exp }), 300)).toBe(true)
  })

  it('returns true for an already-expired token', () => {
    const exp = Math.floor(Date.now() / 1000) - 60
    expect(isTokenExpiringSoon(makeJwt({ exp }), 300)).toBe(true)
  })

  it('returns true for a malformed token', () => {
    expect(isTokenExpiringSoon('garbage', 300)).toBe(true)
  })
})

// ---------------------------------------------------------------------------
// Session storage tests
// ---------------------------------------------------------------------------

const STORAGE_KEY = 'voletu.auth.session'

const fakeSession = {
  accessToken: 'at',
  refreshToken: 'rt',
  user: { id: 'u1', username: 'admin', role: 'ADMIN', displayName: 'Admin' } as any,
}

beforeEach(() => {
  localStorage.clear()
})

describe('loadSession', () => {
  it('returns null when no stored session', () => {
    expect(loadSession()).toBeNull()
  })

  it('returns parsed session from localStorage', () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(fakeSession))

    const result = loadSession()

    expect(result).toEqual(fakeSession)
  })

  it('returns null and clears storage on invalid JSON', () => {
    localStorage.setItem(STORAGE_KEY, '{bad-json')

    const result = loadSession()

    expect(result).toBeNull()
    expect(localStorage.getItem(STORAGE_KEY)).toBeNull()
  })
})

describe('saveSession', () => {
  it('persists session to localStorage', () => {
    saveSession(fakeSession)

    const stored = JSON.parse(localStorage.getItem(STORAGE_KEY)!)
    expect(stored).toEqual(fakeSession)
  })
})

describe('clearSession', () => {
  it('removes session from localStorage', () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(fakeSession))

    clearSession()

    expect(localStorage.getItem(STORAGE_KEY)).toBeNull()
  })
})

// ---------------------------------------------------------------------------
// toSession
// ---------------------------------------------------------------------------

describe('toSession', () => {
  it('maps LoginResponse fields to AuthSession', () => {
    const loginResponse = {
      accessToken: 'access-123',
      refreshToken: 'refresh-456',
      user: { id: 'u2', username: 'user2', role: 'USER', displayName: 'User 2' },
    } as any

    const result = toSession(loginResponse)

    expect(result).toEqual({
      accessToken: 'access-123',
      refreshToken: 'refresh-456',
      user: loginResponse.user,
    })
  })
})
