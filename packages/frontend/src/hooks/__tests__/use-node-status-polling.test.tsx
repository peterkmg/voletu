import { renderHook } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import {
  SYNC_POLL_INTERVAL_ACTIVE_MS,
  SYNC_POLL_INTERVAL_IDLE_MS,
} from '../sync-polling-policy'
import { useNodeStatus } from '../use-node-status'

const useQueryMock = vi.fn()

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
    useAuthStore.setState({ status: 'valid', accessToken: 'token' })
    useNodeStore.getState().setStatus({ isInitialized: true, nodeType: 'PERIPHERAL' })
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
