import { useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { useNodeStore } from '~/stores/node-store'
import { useAuthStore } from '~/stores/auth-store'

function getBaseUrl(): string {
  return (
    (globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
    ?? 'http://127.0.0.1:3000'
  ).replace(/\/+$/, '')
}

interface HealthData {
  status: string
  isInitialized: boolean
  nodeType: string
  nodeName: string
}

interface HealthResponse {
  success: true
  data: HealthData
}

interface NodeStatusData {
  isInitialized: boolean
  nodeType: string
  nodeName: string
  workerState: string
  lastSyncAt: string | null
}

interface NodeStatusResponse {
  success: true
  data: NodeStatusData
}

export function useHealthCheck() {
  const accessToken = useAuthStore(s => s.auth.accessToken)

  const query = useQuery<HealthResponse>({
    queryKey: ['health'],
    queryFn: async () => {
      const res = await fetch(`${getBaseUrl()}/health`)
      if (!res.ok) {
        throw new Error(`Health check failed with status ${res.status}`)
      }
      return res.json() as Promise<HealthResponse>
    },
    refetchInterval: 30_000,
    enabled: !!accessToken,
  })

  useEffect(() => {
    if (query.data?.success) {
      const { isInitialized, nodeType, nodeName } = query.data.data
      useNodeStore.getState().setStatus({
        isInitialized,
        nodeType: nodeType as import('~/stores/node-store').NodeType,
        nodeName,
      })
    }
  }, [query.data])

  return query
}

export function useNodeStatus() {
  const accessToken = useAuthStore(s => s.auth.accessToken)
  const isInitialized = useNodeStore(s => s.status.isInitialized)

  const query = useQuery<NodeStatusResponse>({
    queryKey: ['node', 'status'],
    queryFn: async () => {
      const token = useAuthStore.getState().auth.accessToken
      const res = await fetch(`${getBaseUrl()}/node/status`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })
      if (!res.ok) {
        throw new Error(`Node status request failed with status ${res.status}`)
      }
      return res.json() as Promise<NodeStatusResponse>
    },
    refetchInterval: 10_000,
    enabled: isInitialized && !!accessToken,
  })

  useEffect(() => {
    if (query.data?.success) {
      const { isInitialized, nodeType, nodeName, workerState, lastSyncAt } = query.data.data
      useNodeStore.getState().setStatus({
        isInitialized,
        nodeType: nodeType as import('~/stores/node-store').NodeType,
        nodeName,
        workerState: workerState as import('~/stores/node-store').WorkerState,
        lastSyncAt,
      })
    }
  }, [query.data])

  return query
}
