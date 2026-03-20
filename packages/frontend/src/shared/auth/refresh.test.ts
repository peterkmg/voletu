import { afterEach, describe, expect, it, vi } from 'vitest'
import { createRefreshLock } from './refresh'

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
