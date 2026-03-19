import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getOwnershipTransferColumns(t: TFunction): ColumnDef<OwnershipTransferResponse>[] {
  return [
    {
      id: 'select',
      header: ({ table }) => (
        <Checkbox
          checked={
            table.getIsAllPageRowsSelected()
            || (table.getIsSomePageRowsSelected() && 'indeterminate')
          }
          onCheckedChange={value => table.toggleAllPageRowsSelected(!!value)}
          aria-label="Select all"
          className="translate-y-[2px]"
        />
      ),
      cell: ({ row }) => (
        <Checkbox
          checked={row.getIsSelected()}
          onCheckedChange={value => row.toggleSelected(!!value)}
          aria-label="Select row"
          className="translate-y-[2px]"
        />
      ),
      enableSorting: false,
      enableHiding: false,
    },
    {
      accessorKey: 'id',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.id')}
        />
      ),
      cell: ({ row }) => (
        <span className="font-medium font-mono">
          {row.getValue<string>('id').slice(0, 8)}
        </span>
      ),
    },
    {
      accessorKey: 'date',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:acceptance.columns.date')}
        />
      ),
      cell: ({ row }) => {
        const date = row.getValue<string>('date')
        return (
          <span className="text-muted-foreground text-sm">
            {new Date(date).toLocaleDateString()}
          </span>
        )
      },
    },
    {
      accessorKey: 'status',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.status')}
        />
      ),
      cell: ({ row }) => {
        const status = row.getValue<string>('status')
        return (
          <Badge variant={status === 'Executed' ? 'default' : 'secondary'}>
            {status === 'Executed' ? t('common:status.executed') : t('common:status.draft')}
          </Badge>
        )
      },
    },
    {
      id: 'itemsCount',
      header: t('documents:items.title'),
      cell: ({ row }) => (
        <span className="text-muted-foreground text-sm">
          {row.original.items.length}
        </span>
      ),
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
