import { act, renderHook } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import {
  SYNC_POLL_INTERVAL_ACTIVE_MS,
  SYNC_POLL_INTERVAL_IDLE_MS,
} from '~/hooks/sync-polling-policy'
import { useHealthCheck, useNodeStatus } from '~/hooks/use-node-status'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'

const useQueryMock = vi.fn()
const freshJwt = `x.${btoa(JSON.stringify({ exp: Math.floor(Date.now() / 1000) + 3600 }))}.y`
const refreshedJwt = `x.${btoa(JSON.stringify({ exp: Math.floor(Date.now() / 1000) + 7200 }))}.y`

vi.mock('@tanstack/react-query', () => ({
  useQuery: (...args: unknown[]) => useQueryMock(...args),
}))

vi.mock('~/platform/runtime/health', () => ({
  fetchHealth: vi.fn(),
  applyHealthSnapshot: vi.fn(),
  fetchNodeStatus: vi.fn(),
  applyNodeStatusSnapshot: vi.fn(),
}))

describe('useNodeStatus adaptive polling', () => {
  beforeEach(() => {
    useQueryMock.mockReset()
    useQueryMock.mockReturnValue({ data: undefined })
    useNodeStore.getState().reset()
    useAuthStore.setState({ status: 'valid', accessToken: freshJwt, refreshToken: 'refresh-token' })
    useNodeStore.getState().setStatus({ isInitialized: true, nodeType: 'PERIPHERAL' })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })

  it('uses fast polling while visible for both status and bases queries', () => {
    Object.defineProperty(document, 'visibilityState', { value: 'visible', configurable: true })
    renderHook(() => useNodeStatus())

    expect(useQueryMock).toHaveBeenCalledTimes(2)
    const statusOptions = useQueryMock.mock.calls[0]![0]!
    const basesOptions = useQueryMock.mock.calls[1]![0]!

    expect(statusOptions.refetchInterval()).toBe(SYNC_POLL_INTERVAL_ACTIVE_MS)
    expect(basesOptions.refetchInterval()).toBe(SYNC_POLL_INTERVAL_ACTIVE_MS)
    expect(statusOptions.refetchIntervalInBackground).toBe(true)
    expect(basesOptions.refetchIntervalInBackground).toBe(true)
  })

  it('suppresses global error toasts for health polling', () => {
    renderHook(() => useHealthCheck())

    expect(useQueryMock).toHaveBeenCalledTimes(1)
    const healthOptions = useQueryMock.mock.calls[0]![0]!

    expect(healthOptions.meta).toMatchObject({ suppressErrorToast: true })
  })

  it('suppresses global error toasts for background polling queries', () => {
    renderHook(() => useNodeStatus())

    expect(useQueryMock).toHaveBeenCalledTimes(2)
    const statusOptions = useQueryMock.mock.calls[0]![0]!
    const basesOptions = useQueryMock.mock.calls[1]![0]!

    expect(statusOptions.meta).toMatchObject({ suppressErrorToast: true })
    expect(basesOptions.meta).toMatchObject({ suppressErrorToast: true })
  })

  it('expires the session when node bases polling cannot refresh after 401', async () => {
    useAuthStore.setState({ onUnauthorized: vi.fn(async () => false) } as any)
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({ ok: false, status: 401 }))
    renderHook(() => useNodeStatus())

    const basesOptions = useQueryMock.mock.calls[1]![0]!

    await expect(basesOptions.queryFn()).rejects.toThrow('Session expired')
  })

  it('refreshes and replays node bases polling after 401', async () => {
    const onUnauthorized = vi.fn(async () => {
      useAuthStore.setState({ accessToken: refreshedJwt })
      return true
    })
    useAuthStore.setState({ onUnauthorized } as any)
    vi.stubGlobal(
      'fetch',
      vi.fn()
        .mockResolvedValueOnce(new Response('Unauthorized', { status: 401 }))
        .mockResolvedValueOnce(
          new Response(JSON.stringify({ success: true, data: [{ baseId: 'base-1' }] }), {
            status: 200,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
    )
    renderHook(() => useNodeStatus())

    const basesOptions = useQueryMock.mock.calls[1]![0]!
    let result: unknown
    await act(async () => {
      result = await basesOptions.queryFn()
    })

    expect(onUnauthorized).toHaveBeenCalledTimes(1)
    expect(result).toEqual({ success: true, data: [{ baseId: 'base-1' }] })
  })

  it('uses idle polling while hidden for both status and bases queries', () => {
    Object.defineProperty(document, 'visibilityState', { value: 'hidden', configurable: true })
    renderHook(() => useNodeStatus())

    expect(useQueryMock).toHaveBeenCalledTimes(2)
    const statusOptions = useQueryMock.mock.calls[0]![0]!
    const basesOptions = useQueryMock.mock.calls[1]![0]!

    expect(statusOptions.refetchInterval()).toBe(SYNC_POLL_INTERVAL_IDLE_MS)
    expect(basesOptions.refetchInterval()).toBe(SYNC_POLL_INTERVAL_IDLE_MS)
  })
})
