import type { SyncWatermarkResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { Badge } from '~/components/ui/badge'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '~/components/ui/table'

interface WatermarksTableProps {
  data: SyncWatermarkResponse[]
  isLoading: boolean
}

export function WatermarksTable({ data, isLoading }: WatermarksTableProps) {
  const { t } = useTranslation(['system', 'common'])

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('system:sync.watermarks')}</CardTitle>
      </CardHeader>
      <CardContent>
        {isLoading
          ? (
              <div className="text-muted-foreground">Loading...</div>
            )
          : data.length === 0
            ? (
                <div className="text-muted-foreground">
                  {t('common:table.noResults')}
                </div>
              )
            : (
                <div className="overflow-hidden rounded-md border">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>Target Node</TableHead>
                        <TableHead>Direction</TableHead>
                        <TableHead>Last Audit Log ID</TableHead>
                        <TableHead>{t('system:sync.lastSync')}</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {data.map(wm => (
                        <TableRow key={wm.id}>
                          <TableCell className="font-mono text-sm">
                            {wm.targetNodeId}
                          </TableCell>
                          <TableCell>
                            <Badge
                              variant={
                                wm.direction === 'PULL' ? 'default' : 'outline'
                              }
                              className="text-xs"
                            >
                              {wm.direction}
                            </Badge>
                          </TableCell>
                          <TableCell className="font-mono text-sm">
                            {wm.lastAuditLogId}
                          </TableCell>
                          <TableCell className="text-muted-foreground text-sm">
                            {new Date(wm.syncedAt).toLocaleString()}
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>
              )}
      </CardContent>
    </Card>
  )
}
