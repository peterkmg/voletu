import { useTranslation } from 'react-i18next'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import { cn } from '~/lib/utils'

interface NodeStatusData {
  isInitialized: boolean
  nodeType: string | null
  nodeName: string | null
  workerState: string | null
  lastSyncAt: string | null
}

interface WorkerStatusCardProps {
  data: NodeStatusData | undefined
  isLoading: boolean
}

const STATE_COLORS: Record<string, string> = {
  OnlineIdle: 'bg-green-500',
  Syncing: 'bg-blue-500',
  Backoff: 'bg-yellow-500',
  Sleeping: 'bg-gray-400',
  Offline: 'bg-red-500',
}

function formatRelativeTime(isoString: string): string {
  const diff = Date.now() - new Date(isoString).getTime()
  const seconds = Math.floor(diff / 1000)
  if (seconds < 60)
    return `${seconds}s ago`
  const minutes = Math.floor(seconds / 60)
  if (minutes < 60)
    return `${minutes}m ago`
  const hours = Math.floor(minutes / 60)
  if (hours < 24)
    return `${hours}h ago`
  const days = Math.floor(hours / 24)
  return `${days}d ago`
}

export function WorkerStatusCard({ data, isLoading }: WorkerStatusCardProps) {
  const { t } = useTranslation(['system', 'common'])

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('system:sync.workerState')}</CardTitle>
      </CardHeader>
      <CardContent>
        {isLoading
          ? (
              <div className="text-muted-foreground">{t('common:loading')}</div>
            )
          : data
            ? (
                <dl className="grid grid-cols-1 gap-4 sm:grid-cols-3">
                  <div>
                    <dt className="text-sm font-medium text-muted-foreground">
                      {t('system:sync.workerState')}
                    </dt>
                    <dd className="mt-1 flex items-center gap-2">
                      <span
                        className={cn(
                          'size-2.5 rounded-full',
                          STATE_COLORS[data.workerState ?? ''] ?? 'bg-gray-400',
                        )}
                      />
                      <span className="font-mono text-sm">
                        {data.workerState
                          ? t(`system:sync.states.${data.workerState.charAt(0).toLowerCase() + data.workerState.slice(1)}` as any, data.workerState)
                          : t('system:sync.unknown')}
                      </span>
                    </dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-muted-foreground">
                      {t('system:sync.lastSync')}
                    </dt>
                    <dd className="mt-1 font-mono text-sm">
                      {data.lastSyncAt
                        ? formatRelativeTime(data.lastSyncAt)
                        : t('system:sync.neverSynced')}
                    </dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-muted-foreground">
                      {t('system:node.label')}
                    </dt>
                    <dd className="mt-1 font-mono text-sm">{data.nodeName ?? t('system:sync.unknown')}</dd>
                  </div>
                </dl>
              )
            : (
                <div className="text-muted-foreground">{t('common:noDataAvailable')}</div>
              )}
      </CardContent>
    </Card>
  )
}
