import { useQuery } from '@tanstack/react-query'
import { useEffect } from 'react'
import { client } from '~/api/client'
import {
  applyHealthSnapshot,
  applyNodeStatusSnapshot,
  fetchHealth,
  fetchNodeStatus,
} from '~/platform/runtime/health'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import { getCurrentSyncPollInterval } from './sync-polling-policy'

const backgroundPollingMeta = { suppressErrorToast: true } as const

export function useHealthCheck() {
  const accessToken = useAuthStore(s => s.accessToken)

  const query = useQuery({
    queryKey: ['health'],
    queryFn: () => fetchHealth(),
    refetchInterval: 60_000,
    enabled: !!accessToken,
    meta: backgroundPollingMeta,
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
  const syncRefetchInterval = () => getCurrentSyncPollInterval()

  const query = useQuery({
    queryKey: ['node', 'status'],
    queryFn: () => fetchNodeStatus(),
    refetchInterval: syncRefetchInterval,
    refetchIntervalInBackground: true,
    enabled: isInitialized && !!accessToken,
    meta: backgroundPollingMeta,
  })

  // Also fetch base assignments for peripheral nodes
  const nodeType = useNodeStore(s => s.status.nodeType)
  const basesQuery = useQuery<{ success: true, data: Array<{ baseId: string }> }>({
    queryKey: ['node', 'bases'],
    queryFn: async () => {
      const response = await client<{ success: true, data: Array<{ baseId: string }> }>({
        method: 'GET',
        url: '/node/bases',
      })
      return response.data
    },
    refetchInterval: syncRefetchInterval,
    refetchIntervalInBackground: true,
    enabled: isInitialized && !!accessToken && nodeType === 'PERIPHERAL',
    meta: backgroundPollingMeta,
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
