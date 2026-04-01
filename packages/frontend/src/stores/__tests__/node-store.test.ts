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
      // Unchanged fields stay at defaults
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
  })
})
