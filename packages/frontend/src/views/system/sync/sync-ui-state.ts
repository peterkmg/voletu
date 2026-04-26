import type { NodeStatus } from '~/stores/node-store'

// One enum for every surface that renders "how is the node doing". The sidebar
// badge and the readiness card consume this to stay in lock-step.
export type SyncUiState
  = | 'central' // this node is the central — no setup flow at all
    | 'setupIncomplete' // peripheral, configuration incomplete (first-run)
    | 'online' // peripheral, configured, worker reachable idle
    | 'syncing' // peripheral, configured, worker actively syncing
    | 'offline' // peripheral, configured, worker not reachable right now

export function deriveSyncUiState(status: NodeStatus, basesLoaded: boolean): SyncUiState {
  // Central nodes never enter the setup flow.
  if (status.nodeType === 'CENTRAL')
    return 'central'

  // Node type unknown — bootstrap hasn't resolved the peripheral's identity yet.
  if (status.nodeType === null)
    return 'setupIncomplete'

  // Peripheral known, but the user has not yet run the init wizard.
  if (!status.isInitialized)
    return 'setupIncomplete'

  // Initialized, but we have not yet confirmed the bases list from the server.
  if (!basesLoaded)
    return 'setupIncomplete'

  // Bases known — if none assigned, setup is genuinely incomplete.
  if (status.assignedBaseIds.length === 0)
    return 'setupIncomplete'

  // Setup is complete. Everything else is a view of worker reachability.
  if (status.workerState === 'Syncing')
    return 'syncing'
  if (status.workerState === 'OnlineIdle')
    return 'online'
  return 'offline'
}

export function isSetupComplete(status: NodeStatus, basesLoaded: boolean): boolean {
  if (status.nodeType !== 'PERIPHERAL')
    return false
  if (!status.isInitialized)
    return false
  if (!basesLoaded)
    return false
  return status.assignedBaseIds.length > 0
}
