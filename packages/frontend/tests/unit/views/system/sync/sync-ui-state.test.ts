import type { NodeStatus } from '~/stores/node-store'
import type { SyncUiState } from '~/views/system/sync/sync-ui-state'
import { deriveSyncUiState, isSetupComplete } from '~/views/system/sync/sync-ui-state'

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
      expect(
        deriveSyncUiState(
          status({ nodeType: 'CENTRAL', isInitialized: false, assignedBaseIds: [] }),
          false,
        ),
      ).toBe('central')
    })
  })

  describe('bootstrap not yet resolved (nodeType null)', () => {
    it('returns "setupIncomplete" when nodeType is null and uninitialized', () => {
      expect(deriveSyncUiState(status(), false)).toBe('setupIncomplete')
    })

    it('returns "setupIncomplete" when nodeType is null even if basesLoaded is true', () => {
      expect(deriveSyncUiState(status(), true)).toBe('setupIncomplete')
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
    it('returns "setupIncomplete" even when assignedBaseIds is empty', () => {
      expect(
        deriveSyncUiState(
          status({
            nodeType: 'PERIPHERAL',
            isInitialized: true,
            assignedBaseIds: [],
          }),
          false,
        ),
      ).toBe('setupIncomplete')
    })

    it('returns "offline" with non-empty assignedBaseIds even before the secondary bases query confirms', () => {
      expect(
        deriveSyncUiState(
          status({
            nodeType: 'PERIPHERAL',
            isInitialized: true,
            assignedBaseIds: ['b1'],
          }),
          false,
        ),
      ).toBe('offline')
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

    it('bootstrap window with bases stored locally still shows setupIncomplete', () => {
      const s = status({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
        assignedBaseIds: [],
      })
      const result: SyncUiState = deriveSyncUiState(s, false)
      expect(result).toBe('setupIncomplete')
      expect(result).not.toBe('offline')
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

  it('returns true with assigned bases even before the secondary bases query confirms', () => {
    expect(
      isSetupComplete(
        status({ nodeType: 'PERIPHERAL', isInitialized: true, assignedBaseIds: ['b1'] }),
        false,
      ),
    ).toBe(true)
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
