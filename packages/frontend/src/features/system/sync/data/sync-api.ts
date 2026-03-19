import { syncAuditLogListQueryKey } from '~/generated/hooks/SyncHooks/useSyncAuditLogList'
import { syncStatusQueryKey } from '~/generated/hooks/SyncHooks/useSyncStatus'
import { syncWatermarkListQueryKey } from '~/generated/hooks/SyncHooks/useSyncWatermarkList'
import { queryClient } from '~/shared/api/query-client'

export type { AuditLogResponse } from '~/generated/types/AuditLogResponse'
export type { SyncStatusResponse } from '~/generated/types/SyncStatusResponse'
export type { SyncWatermarkResponse } from '~/generated/types/SyncWatermarkResponse'

export function invalidateSyncStatus() {
  return queryClient.invalidateQueries({ queryKey: syncStatusQueryKey() })
}

export function invalidateSyncWatermarks() {
  return queryClient.invalidateQueries({ queryKey: syncWatermarkListQueryKey() })
}

export function invalidateAuditLogs() {
  return queryClient.invalidateQueries({ queryKey: syncAuditLogListQueryKey() })
}
