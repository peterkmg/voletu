import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AuditLogResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable } from '~/components/data-table'
import { DataTableColumnHeader } from '~/components/data-table/column-header'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Badge } from '~/components/ui/badge'
import { Skeleton } from '~/components/ui/skeleton'
import { useSyncAuditLogList } from '~/generated/hooks/SyncHooks/useSyncAuditLogList'
import { usePageTitle } from '~/hooks/use-page-title'

const route = getRouteApi('/_authenticated/system/audit-logs/')
const globalFilterFn = createGlobalFilter<AuditLogResponse>('tableName', 'recordId', 'action')

function actionVariant(action: string) {
  switch (action) {
    case 'INSERT': return 'default' as const
    case 'UPDATE': return 'outline' as const
    case 'HARD_DELETE': return 'destructive' as const
    default: return 'secondary' as const
  }
}

function formatTableName(name: string) {
  return name.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase())
}

function getAuditLogColumns(_t: TFunction): ColumnDef<AuditLogResponse>[] {
  return [
    dateColumn<AuditLogResponse>('timestamp', 'Timestamp'),
    {
      accessorKey: 'tableName',
      minSize: 160,
      header: ({ column }) => <DataTableColumnHeader column={column} title="Table" />,
      meta: { label: 'Table', sizingCategory: 'flex' as const },
      cell: ({ row }) => (
        <span className="font-medium">{formatTableName(row.getValue('tableName'))}</span>
      ),
    },
    {
      accessorKey: 'action',
      minSize: 90,
      maxSize: 110,
      header: ({ column }) => <DataTableColumnHeader column={column} title="Action" />,
      meta: { label: 'Action', sizingCategory: 'capped' as const },
      cell: ({ row }) => (
        <Badge variant={actionVariant(row.getValue('action'))} className="text-xs">
          {row.getValue<string>('action')}
        </Badge>
      ),
    },
    {
      accessorKey: 'recordId',
      minSize: 200,
      header: ({ column }) => <DataTableColumnHeader column={column} title="Record ID" />,
      meta: { label: 'Record ID', sizingCategory: 'flex' as const },
      cell: ({ row }) => (
        <span className="font-mono text-xs text-muted-foreground">{row.getValue('recordId')}</span>
      ),
    },
    {
      accessorKey: 'targetBaseIds',
      minSize: 80,
      maxSize: 110,
      header: ({ column }) => <DataTableColumnHeader column={column} title="Routing" />,
      meta: { label: 'Routing', sizingCategory: 'capped' as const },
      cell: ({ row }) => {
        const val = row.getValue<string>('targetBaseIds')
        if (!val)
          return <span className="text-muted-foreground">{'\u2014'}</span>
        const count = val.split(',').filter(Boolean).length
        return (
          <span className="text-xs text-muted-foreground">
            {count}
            {' '}
            {count === 1 ? 'base' : 'bases'}
          </span>
        )
      },
    },
    {
      accessorKey: 'userRoleWeight',
      minSize: 60,
      maxSize: 70,
      header: ({ column }) => <DataTableColumnHeader column={column} title="Weight" />,
      meta: { label: 'Weight', sizingCategory: 'fixed' as const, align: 'right' as const },
      cell: ({ row }) => (
        <span className="text-xs tabular-nums text-muted-foreground">{row.getValue('userRoleWeight')}</span>
      ),
    },
  ]
}

export function AuditLogsPage() {
  const { t } = useTranslation(['system', 'common'])
  usePageTitle(t('system:sync.auditLogs'))

  const { data: auditLogsData, isLoading } = useSyncAuditLogList({
    query: { refetchInterval: 30_000 },
  })
  const data = auditLogsData?.data ?? []

  return (
    <>
      <Header fixed />
      <Main fixed className="flex flex-1 flex-col gap-4">
        {isLoading
          ? (
              <div className="flex flex-1 flex-col gap-4">
                <Skeleton className="h-9 w-64" />
                <div className="flex-1 rounded-md border">
                  <div className="space-y-3 p-4">
                    {Array.from({ length: 8 }, (_, i) => (
                      <Skeleton key={i} className="h-10 w-full" />
                    ))}
                  </div>
                </div>
              </div>
            )
          : (
              <EntityTable
                data={data}
                getColumns={getAuditLogColumns}
                routeApi={route}
                globalFilterFn={globalFilterFn}
                i18nNamespaces={['system', 'common']}
                tableId="audit-logs"
              />
            )}
      </Main>
    </>
  )
}
