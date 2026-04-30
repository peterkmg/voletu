import type { NodeStatus } from '~/stores/node-store'

export type SyncUiState
  = | 'central'
    | 'setupIncomplete'
    | 'online'
    | 'syncing'
    | 'offline'

export function deriveSyncUiState(status: NodeStatus, _basesLoaded: boolean): SyncUiState {
  if (status.nodeType === 'CENTRAL')
    return 'central'

  if (status.nodeType === null)
    return 'setupIncomplete'

  if (!status.isInitialized)
    return 'setupIncomplete'

  if (status.assignedBaseIds.length === 0)
    return 'setupIncomplete'

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
  if (!basesLoaded && status.assignedBaseIds.length === 0)
    return false
  return status.assignedBaseIds.length > 0
}
