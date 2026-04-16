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
  // Flipped to true once the bases list has been fetched at least once (even if empty).
  // Lets consumers distinguish "default zero state" from "server confirmed: no bases".
  basesLoaded: boolean
  // Sticky: set true the first time the worker is observed in a reachable state
  // (OnlineIdle | Syncing), and never unset within the session. The readiness
  // checklist's "central verified" step reads this so a transient offline does
  // not flip a completed step back to unchecked.
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
