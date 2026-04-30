import { create } from 'zustand'

export type NodeType = 'CENTRAL' | 'PERIPHERAL'

export type WorkerState = 'Sleeping' | 'Offline' | 'OnlineIdle' | 'Syncing' | 'Backoff'

export interface NodeStatus {
  isInitialized: boolean
  nodeType: NodeType | null
  nodeName: string | null
  workerState: WorkerState | null
  lastSyncAt: string | null
  assignedBaseIds: string[]
  centralApiUrl: string | null
}

const defaultStatus: NodeStatus = {
  isInitialized: false,
  nodeType: null,
  nodeName: null,
  workerState: null,
  lastSyncAt: null,
  assignedBaseIds: [],
  centralApiUrl: null,
}

interface NodeStore {
  status: NodeStatus

  basesLoaded: boolean

  centralVerifiedOnce: boolean
  setStatus: (status: Partial<NodeStatus>) => void
  markBasesLoaded: () => void
  reset: () => void
}

function isReachable(workerState: WorkerState | null): boolean {
  return workerState === 'OnlineIdle' || workerState === 'Syncing'
}

export const useNodeStore = create<NodeStore>()(set => ({
  status: { ...defaultStatus },
  basesLoaded: false,
  centralVerifiedOnce: false,

  setStatus: (partial: Partial<NodeStatus>) => {
    set((state) => {
      const nextStatus = { ...state.status, ...partial }
      const nextVerified = state.centralVerifiedOnce || isReachable(nextStatus.workerState)
      return {
        status: nextStatus,
        centralVerifiedOnce: nextVerified,
      }
    })
  },

  markBasesLoaded: () => {
    set({ basesLoaded: true })
  },

  reset: () => {
    set({
      status: { ...defaultStatus },
      basesLoaded: false,
      centralVerifiedOnce: false,
    })
  },
}))
