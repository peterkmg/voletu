import { useNodeStore } from '~/stores/node-store'

beforeEach(() => {
  useNodeStore.getState().reset()
})

describe('node-store', () => {
  describe('initial state', () => {
    it('starts with all-null defaults and isInitialized = false', () => {
      const { status } = useNodeStore.getState()
      expect(status.isInitialized).toBe(false)
      expect(status.nodeType).toBeNull()
      expect(status.nodeName).toBeNull()
      expect(status.workerState).toBeNull()
      expect(status.lastSyncAt).toBeNull()
    })
  })

  describe('setStatus()', () => {
    it('merges a partial update into existing status', () => {
      useNodeStore.getState().setStatus({ nodeType: 'CENTRAL', nodeName: 'node-1' })

      const { status } = useNodeStore.getState()
      expect(status.nodeType).toBe('CENTRAL')
      expect(status.nodeName).toBe('node-1')

      expect(status.isInitialized).toBe(false)
      expect(status.workerState).toBeNull()
    })

    it('overwrites previously set fields', () => {
      useNodeStore.getState().setStatus({ workerState: 'OnlineIdle' })
      useNodeStore.getState().setStatus({ workerState: 'Syncing' })

      expect(useNodeStore.getState().status.workerState).toBe('Syncing')
    })

    it('can set isInitialized to true', () => {
      useNodeStore.getState().setStatus({ isInitialized: true })
      expect(useNodeStore.getState().status.isInitialized).toBe(true)
    })

    it('can update lastSyncAt', () => {
      const now = new Date().toISOString()
      useNodeStore.getState().setStatus({ lastSyncAt: now })
      expect(useNodeStore.getState().status.lastSyncAt).toBe(now)
    })

    it('handles multiple fields in one call', () => {
      useNodeStore.getState().setStatus({
        isInitialized: true,
        nodeType: 'PERIPHERAL',
        nodeName: 'edge-1',
        workerState: 'Sleeping',
        lastSyncAt: '2026-01-01T00:00:00Z',
      })

      const { status } = useNodeStore.getState()
      expect(status.isInitialized).toBe(true)
      expect(status.nodeType).toBe('PERIPHERAL')
      expect(status.nodeName).toBe('edge-1')
      expect(status.workerState).toBe('Sleeping')
      expect(status.lastSyncAt).toBe('2026-01-01T00:00:00Z')
    })
  })

  describe('reset()', () => {
    it('returns all fields to their defaults', () => {
      useNodeStore.getState().setStatus({
        isInitialized: true,
        nodeType: 'CENTRAL',
        nodeName: 'node-1',
        workerState: 'Syncing',
        lastSyncAt: '2026-01-01T00:00:00Z',
      })

      useNodeStore.getState().reset()

      const { status } = useNodeStore.getState()
      expect(status.isInitialized).toBe(false)
      expect(status.nodeType).toBeNull()
      expect(status.nodeName).toBeNull()
      expect(status.workerState).toBeNull()
      expect(status.lastSyncAt).toBeNull()
    })

    it('allows setStatus after reset', () => {
      useNodeStore.getState().setStatus({ nodeType: 'CENTRAL' })
      useNodeStore.getState().reset()
      useNodeStore.getState().setStatus({ nodeType: 'PERIPHERAL' })

      expect(useNodeStore.getState().status.nodeType).toBe('PERIPHERAL')
    })

    it('resets basesLoaded and centralVerifiedOnce', () => {
      useNodeStore.getState().markBasesLoaded()
      useNodeStore.getState().setStatus({ workerState: 'OnlineIdle' })
      expect(useNodeStore.getState().basesLoaded).toBe(true)
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(true)

      useNodeStore.getState().reset()

      expect(useNodeStore.getState().basesLoaded).toBe(false)
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(false)
    })
  })

  describe('basesLoaded flag', () => {
    it('defaults to false', () => {
      expect(useNodeStore.getState().basesLoaded).toBe(false)
    })

    it('flips true after markBasesLoaded()', () => {
      useNodeStore.getState().markBasesLoaded()
      expect(useNodeStore.getState().basesLoaded).toBe(true)
    })

    it('stays true once marked; setStatus does not unset it', () => {
      useNodeStore.getState().markBasesLoaded()
      useNodeStore.getState().setStatus({ assignedBaseIds: [] })
      expect(useNodeStore.getState().basesLoaded).toBe(true)
    })
  })

  describe('centralVerifiedOnce flag', () => {
    it('defaults to false', () => {
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(false)
    })

    it('flips true when workerState transitions through OnlineIdle', () => {
      useNodeStore.getState().setStatus({ workerState: 'OnlineIdle' })
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(true)
    })

    it('flips true when workerState transitions through Syncing', () => {
      useNodeStore.getState().setStatus({ workerState: 'Syncing' })
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(true)
    })

    it('does NOT unset once true when worker later goes Offline', () => {
      useNodeStore.getState().setStatus({ workerState: 'OnlineIdle' })
      useNodeStore.getState().setStatus({ workerState: 'Offline' })
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(true)
    })

    it('does NOT unset once true when worker later goes Backoff', () => {
      useNodeStore.getState().setStatus({ workerState: 'Syncing' })
      useNodeStore.getState().setStatus({ workerState: 'Backoff' })
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(true)
    })

    it('stays false when worker never reached a reachable state', () => {
      useNodeStore.getState().setStatus({ workerState: 'Offline' })
      useNodeStore.getState().setStatus({ workerState: 'Sleeping' })
      useNodeStore.getState().setStatus({ workerState: 'Backoff' })
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(false)
    })

    it('stays false when workerState is null', () => {
      useNodeStore.getState().setStatus({ workerState: null })
      expect(useNodeStore.getState().centralVerifiedOnce).toBe(false)
    })
  })
})
