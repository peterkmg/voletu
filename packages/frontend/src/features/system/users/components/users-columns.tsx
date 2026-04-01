import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { UserResponse } from '~/generated/types'
import { actionsColumn, DataTableColumnHeader, dateColumn, selectColumn, textColumn } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { createRowActions } from '~/lib/create-row-actions'
import { useUsers } from './users-provider'

const DataTableRowActions = createRowActions<UserResponse>({ useEntity: useUsers, deleteOnly: true })

export function getUserColumns(t: TFunction): ColumnDef<UserResponse>[] {
  return [
    selectColumn<UserResponse>(),
    textColumn<UserResponse>('username', t('system:users.columns.username')),
    textColumn<UserResponse>('fullname', t('system:users.columns.fullname'), { primary: false }),
    {
      accessorKey: 'role',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:users.columns.role')}
        />
      ),
      cell: ({ row }) => (
        <Badge variant="outline" className="text-xs">
          {row.getValue('role')}
        </Badge>
      ),
    },
    dateColumn<UserResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<UserResponse>(DataTableRowActions),
  ]
}
