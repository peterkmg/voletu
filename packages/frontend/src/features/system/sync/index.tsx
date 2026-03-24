import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useSyncAuditLogList } from '~/generated/hooks/SyncHooks/useSyncAuditLogList'
import { useSyncStatus } from '~/generated/hooks/SyncHooks/useSyncStatus'
import { useSyncWatermarkList } from '~/generated/hooks/SyncHooks/useSyncWatermarkList'
import { useNodeStatus } from '~/shared/api/hooks/use-node-status'
import { AuditLogTable } from './components/audit-log-table'
import { SyncStatusCard } from './components/sync-status-card'
import { WatermarksTable } from './components/watermarks-table'
import { WorkerStatusCard } from './components/worker-status-card'

export function SyncDashboard() {
  const { t } = useTranslation(['system'])

  const { data: statusData, isLoading: statusLoading } = useSyncStatus()
  const status = statusData?.data

  const { data: nodeStatusData, isLoading: nodeStatusLoading } = useNodeStatus()
  const nodeStatus = nodeStatusData?.data

  const { data: watermarksData, isLoading: watermarksLoading } = useSyncWatermarkList()
  const watermarks = watermarksData?.data ?? []

  const { data: auditLogsData, isLoading: auditLogsLoading } = useSyncAuditLogList({ query: { refetchInterval: 30_000 } })
  const auditLogs = auditLogsData?.data ?? []

  return (
    <>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('system:sync.title')}
            </h2>
          </div>
        </div>

        <WorkerStatusCard data={nodeStatus} isLoading={nodeStatusLoading} />
        <SyncStatusCard data={status} isLoading={statusLoading} />
        <WatermarksTable data={watermarks} isLoading={watermarksLoading} />
        <AuditLogTable data={auditLogs} isLoading={auditLogsLoading} />
      </Main>
    </>
  )
}
