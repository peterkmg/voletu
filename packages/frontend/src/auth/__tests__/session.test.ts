import { afterEach, describe, expect, it, vi } from 'vitest'
import { createRefreshLock, decodeJwtExp, isTokenExpiringSoon } from '../session'

// ---------------------------------------------------------------------------
// JWT decode tests (formerly jwt-decode.test.ts)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Refresh lock tests (formerly refresh.test.ts)
// ---------------------------------------------------------------------------

describe('createRefreshLock', () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('calls the refresh function and returns the new session', async () => {
    const mockSession = { accessToken: 'new-at', refreshToken: 'new-rt', user: { id: '1', username: 'admin' } }
    const refreshFn = vi.fn().mockResolvedValue(mockSession)
    const lock = createRefreshLock(refreshFn)

    const result = await lock.acquire('old-refresh-token')
    expect(refreshFn).toHaveBeenCalledWith('old-refresh-token')
    expect(result).toEqual(mockSession)
  })

  it('deduplicates concurrent calls — refresh function is called only once', async () => {
    const mockSession = { accessToken: 'new-at', refreshToken: 'new-rt', user: { id: '1', username: 'admin' } }
    const refreshFn = vi.fn().mockResolvedValue(mockSession)
    const lock = createRefreshLock(refreshFn)

    const [r1, r2, r3] = await Promise.all([
      lock.acquire('old-rt'),
      lock.acquire('old-rt'),
      lock.acquire('old-rt'),
    ])

    expect(refreshFn).toHaveBeenCalledTimes(1)
    expect(r1).toEqual(mockSession)
    expect(r2).toEqual(mockSession)
    expect(r3).toEqual(mockSession)
  })

  it('allows a new call after the previous one resolved', async () => {
    let callCount = 0
    const refreshFn = vi.fn().mockImplementation(async () => {
      callCount++
      return { accessToken: `at-${callCount}`, refreshToken: `rt-${callCount}`, user: { id: '1', username: 'admin' } }
    })
    const lock = createRefreshLock(refreshFn)

    const r1 = await lock.acquire('rt-1')
    const r2 = await lock.acquire('rt-2')

    expect(refreshFn).toHaveBeenCalledTimes(2)
    expect(r1.accessToken).toBe('at-1')
    expect(r2.accessToken).toBe('at-2')
  })

  it('propagates errors and resets the lock', async () => {
    const refreshFn = vi.fn().mockRejectedValue(new Error('refresh failed'))
    const lock = createRefreshLock(refreshFn)

    await expect(lock.acquire('bad-rt')).rejects.toThrow('refresh failed')

    // Lock should be reset — next call should try again
    refreshFn.mockResolvedValue({ accessToken: 'new', refreshToken: 'new', user: { id: '1', username: 'admin' } })
    const result = await lock.acquire('good-rt')
    expect(result.accessToken).toBe('new')
    expect(refreshFn).toHaveBeenCalledTimes(2)
  })
})
