import { describe, expect, it } from 'vitest'
import { decodeJwtExp, isTokenExpiringSoon } from './jwt-decode'

// Helper: build a minimal JWT with a given exp claim
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
