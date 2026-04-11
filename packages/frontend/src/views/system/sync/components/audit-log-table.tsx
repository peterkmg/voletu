import type { AuditLogResponse } from '~/generated/types'
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

interface AuditLogTableProps {
  data: AuditLogResponse[]
  isLoading: boolean
}

function actionVariant(action: string) {
  switch (action) {
    case 'Create':
      return 'default' as const
    case 'Update':
      return 'outline' as const
    case 'Delete':
      return 'destructive' as const
    default:
      return 'secondary' as const
  }
}

export function AuditLogTable({ data, isLoading }: AuditLogTableProps) {
  const { t } = useTranslation(['system', 'common'])

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('system:sync.auditLogs')}</CardTitle>
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
                        <TableHead>{t('common:table.id')}</TableHead>
                        <TableHead>Table</TableHead>
                        <TableHead>Record ID</TableHead>
                        <TableHead>Action</TableHead>
                        <TableHead>User</TableHead>
                        <TableHead>Timestamp</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {data.map(log => (
                        <TableRow key={log.id}>
                          <TableCell className="font-mono text-sm">
                            {log.id}
                          </TableCell>
                          <TableCell className="text-sm">
                            {log.tableName}
                          </TableCell>
                          <TableCell className="font-mono text-sm">
                            {log.recordId}
                          </TableCell>
                          <TableCell>
                            <Badge
                              variant={actionVariant(log.action)}
                              className="text-xs"
                            >
                              {log.action}
                            </Badge>
                          </TableCell>
                          <TableCell className="font-mono text-sm">
                            {log.userId}
                          </TableCell>
                          <TableCell className="text-muted-foreground text-sm">
                            {new Date(log.timestamp).toLocaleString()}
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
