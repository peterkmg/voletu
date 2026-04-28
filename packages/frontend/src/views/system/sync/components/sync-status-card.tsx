import type { SyncStatusResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'

interface SyncStatusCardProps {
  data: SyncStatusResponse | undefined
  isLoading: boolean
}

export function SyncStatusCard({ data, isLoading }: SyncStatusCardProps) {
  const { t } = useTranslation(['system', 'common'])

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('system:sync.status')}</CardTitle>
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
                      {t('system:sync.nodeId')}
                    </dt>
                    <dd className="mt-1 font-mono text-sm">{data.nodeId}</dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-muted-foreground">
                      {t('system:sync.nodeType')}
                    </dt>
                    <dd className="mt-1 font-mono text-sm">{data.nodeType}</dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-muted-foreground">
                      {t('system:sync.highestAuditLogId')}
                    </dt>
                    <dd className="mt-1 font-mono text-sm">
                      {data.highestAuditLogId}
                    </dd>
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
