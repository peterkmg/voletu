import { useQuery } from '@tanstack/react-query'
import { useEffect } from 'react'
import { getApiBaseUrl } from '~/platform/runtime/api-base-url'
import {
  applyHealthSnapshot,
  applyNodeStatusSnapshot,
  fetchHealth,
  fetchNodeStatus,
} from '~/platform/runtime/health'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'

export function useHealthCheck() {
  const accessToken = useAuthStore(s => s.accessToken)

  const query = useQuery({
    queryKey: ['health'],
    queryFn: () => fetchHealth(),
    refetchInterval: 60_000,
    enabled: !!accessToken,
  })

  useEffect(() => {
    if (query.data) {
      applyHealthSnapshot(query.data)
    }
  }, [query.data])

  return query
}

export function useNodeStatus() {
  const accessToken = useAuthStore(s => s.accessToken)
  const isInitialized = useNodeStore(s => s.status.isInitialized)

  const query = useQuery({
    queryKey: ['node', 'status'],
    queryFn: () => fetchNodeStatus(),
    refetchInterval: 30_000,
    enabled: isInitialized && !!accessToken,
  })

  // Also fetch base assignments for peripheral nodes
  const nodeType = useNodeStore(s => s.status.nodeType)
  const basesQuery = useQuery<{ success: true, data: Array<{ baseId: string }> }>({
    queryKey: ['node', 'bases'],
    queryFn: async () => {
      const token = useAuthStore.getState().accessToken
      const res = await fetch(`${getApiBaseUrl()}/node/bases`, {
        headers: { Authorization: `Bearer ${token}` },
      })
      if (!res.ok)
        throw new Error('Failed to fetch node bases')
      return res.json()
    },
    refetchInterval: 30_000,
    enabled: isInitialized && !!accessToken && nodeType === 'PERIPHERAL',
  })

  useEffect(() => {
    if (query.data) {
      applyNodeStatusSnapshot(query.data)
    }
  }, [query.data])

  useEffect(() => {
    if (basesQuery.data?.success) {
      useNodeStore.getState().setStatus({
        assignedBaseIds: basesQuery.data.data.map(b => b.baseId),
      })
      useNodeStore.getState().markBasesLoaded()
    }
  }, [basesQuery.data])

  return query
}
