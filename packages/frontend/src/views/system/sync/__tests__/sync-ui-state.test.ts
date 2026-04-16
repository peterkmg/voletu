import type { SyncUiState } from '../sync-ui-state'
import type { NodeStatus } from '~/stores/node-store'
import { deriveSyncUiState, isSetupComplete } from '../sync-ui-state'

// Helper: build a NodeStatus with sensible defaults; override fields per test.
function status(overrides: Partial<NodeStatus> = {}): NodeStatus {
  return {
    isInitialized: false,
    nodeType: null,
    nodeName: null,
    workerState: null,
    lastSyncAt: null,
    assignedBaseIds: [],
    centralApiUrl: null,
    ...overrides,
  }
}

describe('deriveSyncUiState', () => {
  describe('central nodes', () => {
    it.each([true, false])('returns "central" regardless of basesLoaded=%s', (basesLoaded) => {
      expect(deriveSyncUiState(status({ nodeType: 'CENTRAL' }), basesLoaded)).toBe('central')
    })

    it('returns "central" even if other fields are inconsistent', () => {
      // Shouldn't happen in practice but the central branch is short-circuit.
      expect(
        deriveSyncUiState(
          status({ nodeType: 'CENTRAL', isInitialized: false, assignedBaseIds: [] }),
          false,
        ),
      ).toBe('central')
    })
  })

  describe('bootstrap not yet resolved (nodeType null)', () => {
    it('returns "setupLoading" when nodeType is null and uninitialized', () => {
      expect(deriveSyncUiState(status(), false)).toBe('setupLoading')
    })

    it('returns "setupLoading" when nodeType is null even if basesLoaded is true', () => {
      expect(deriveSyncUiState(status(), true)).toBe('setupLoading')
    })
  })

  describe('peripheral not yet initialized', () => {
    it('returns "setupIncomplete" when peripheral known but init incomplete', () => {
      expect(
        deriveSyncUiState(status({ nodeType: 'PERIPHERAL', isInitialized: false }), false),
      ).toBe('setupIncomplete')
    })

    it('returns "setupIncomplete" regardless of basesLoaded when init is incomplete', () => {
      expect(
        deriveSyncUiState(status({ nodeType: 'PERIPHERAL', isInitialized: false }), true),
      ).toBe('setupIncomplete')
    })
  })

  describe('peripheral initialized, bases not yet loaded', () => {
    it('returns "setupLoading" even when assignedBaseIds is empty', () => {
      // This is the critical case: default empty bases must NOT be read as "setup incomplete".
      expect(
        deriveSyncUiState(
          status({
            nodeType: 'PERIPHERAL',
            isInitialized: true,
            assignedBaseIds: [],
          }),
          false,
        ),
      ).toBe('setupLoading')
    })

    it('returns "setupLoading" even with non-empty assignedBaseIds until flag flips', () => {
      // Even if someone populated bases via a different path, we wait for the explicit flag.
      expect(
        deriveSyncUiState(
          status({
            nodeType: 'PERIPHERAL',
            isInitialized: true,
            assignedBaseIds: ['b1'],
          }),
          false,
        ),
      ).toBe('setupLoading')
    })
  })

  describe('peripheral initialized, bases loaded but none assigned', () => {
    it('returns "setupIncomplete"', () => {
      expect(
        deriveSyncUiState(
          status({
            nodeType: 'PERIPHERAL',
            isInitialized: true,
            assignedBaseIds: [],
          }),
          true,
        ),
      ).toBe('setupIncomplete')
    })
  })

  describe('setup complete (peripheral + initialized + basesLoaded + >=1 base)', () => {
    const base = (workerState: NodeStatus['workerState']): NodeStatus => status({
      nodeType: 'PERIPHERAL',
      isInitialized: true,
      assignedBaseIds: ['b1'],
      workerState,
    })

    it('returns "syncing" when worker is Syncing', () => {
      expect(deriveSyncUiState(base('Syncing'), true)).toBe('syncing')
    })

    it('returns "online" when worker is OnlineIdle', () => {
      expect(deriveSyncUiState(base('OnlineIdle'), true)).toBe('online')
    })

    it('returns "offline" when worker is Offline', () => {
      expect(deriveSyncUiState(base('Offline'), true)).toBe('offline')
    })

    it('returns "offline" when worker is Backoff', () => {
      expect(deriveSyncUiState(base('Backoff'), true)).toBe('offline')
    })

    it('returns "offline" when worker is Sleeping', () => {
      expect(deriveSyncUiState(base('Sleeping'), true)).toBe('offline')
    })

    it('returns "offline" when workerState is null (not yet reported)', () => {
      // Critical: after setup is complete, a transient "we don't know" must NOT regress to "setup needed".
      expect(deriveSyncUiState(base(null), true)).toBe('offline')
    })
  })

  describe('regression guards', () => {
    it('"central connection down" after setup is offline, NOT setupIncomplete', () => {
      const s = status({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
        assignedBaseIds: ['b1', 'b2'],
        workerState: 'Backoff',
      })
      const result: SyncUiState = deriveSyncUiState(s, true)
      expect(result).toBe('offline')
      expect(result).not.toBe('setupIncomplete')
    })

    it('bootstrap window with bases stored locally shows setupLoading, not setupIncomplete', () => {
      const s = status({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
        assignedBaseIds: [], // default — bases query hasn't resolved yet
      })
      const result: SyncUiState = deriveSyncUiState(s, false)
      expect(result).toBe('setupLoading')
      expect(result).not.toBe('setupIncomplete')
    })
  })
})

describe('isSetupComplete', () => {
  it('returns false for central nodes', () => {
    expect(isSetupComplete(status({ nodeType: 'CENTRAL', isInitialized: true }), true)).toBe(false)
  })

  it('returns false when nodeType is null', () => {
    expect(isSetupComplete(status({ isInitialized: true, assignedBaseIds: ['b1'] }), true)).toBe(false)
  })

  it('returns false when not initialized', () => {
    expect(
      isSetupComplete(
        status({ nodeType: 'PERIPHERAL', isInitialized: false, assignedBaseIds: ['b1'] }),
        true,
      ),
    ).toBe(false)
  })

  it('returns false when basesLoaded is false', () => {
    expect(
      isSetupComplete(
        status({ nodeType: 'PERIPHERAL', isInitialized: true, assignedBaseIds: ['b1'] }),
        false,
      ),
    ).toBe(false)
  })

  it('returns false when no bases assigned', () => {
    expect(
      isSetupComplete(
        status({ nodeType: 'PERIPHERAL', isInitialized: true, assignedBaseIds: [] }),
        true,
      ),
    ).toBe(false)
  })

  it('returns true when peripheral + initialized + basesLoaded + at least one base', () => {
    expect(
      isSetupComplete(
        status({ nodeType: 'PERIPHERAL', isInitialized: true, assignedBaseIds: ['b1'] }),
        true,
      ),
    ).toBe(true)
  })
})
