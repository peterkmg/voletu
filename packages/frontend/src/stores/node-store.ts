import { create } from 'zustand'

export type NodeType = 'CENTRAL' | 'PERIPHERAL'

export type WorkerState = 'Sleeping' | 'Offline' | 'OnlineIdle' | 'Syncing' | 'Backoff'

export interface NodeStatus {
  isInitialized: boolean
  nodeType: NodeType | null
  nodeName: string | null
  workerState: WorkerState | null
  lastSyncAt: string | null
}

const defaultStatus: NodeStatus = {
  isInitialized: false,
  nodeType: null,
  nodeName: null,
  workerState: null,
  lastSyncAt: null,
}

interface NodeStore {
  status: NodeStatus
  setStatus: (status: Partial<NodeStatus>) => void
  reset: () => void
}

export const useNodeStore = create<NodeStore>()(set => ({
  status: { ...defaultStatus },

  setStatus: (partial: Partial<NodeStatus>) => {
    set(state => ({
      status: { ...state.status, ...partial },
    }))
  },

  reset: () => {
    set({ status: { ...defaultStatus } })
  },
}))
