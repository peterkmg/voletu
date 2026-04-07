import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useSyncStatus } from '~/generated/hooks/SyncHooks/useSyncStatus'
import { useSyncWatermarkList } from '~/generated/hooks/SyncHooks/useSyncWatermarkList'
import { useNodeStatus } from '~/hooks/use-node-status'
import { SyncReadinessCard } from './components/sync-readiness-card'
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

  return (
    <>
      <Header fixed />

      <Main fixed className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('system:sync.title')}
            </h2>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto -mx-4 px-4 space-y-4 sm:space-y-6">
          <SyncReadinessCard />
          <WorkerStatusCard data={nodeStatus} isLoading={nodeStatusLoading} />
          <SyncStatusCard data={status} isLoading={statusLoading} />
          <WatermarksTable data={watermarks} isLoading={watermarksLoading} />
        </div>
      </Main>
    </>
  )
}
