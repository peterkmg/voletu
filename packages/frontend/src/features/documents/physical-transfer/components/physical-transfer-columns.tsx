import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getPhysicalTransferColumns(t: TFunction): ColumnDef<PhysicalTransferResponse>[] {
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
      accessorKey: 'documentNumber',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:acceptance.columns.documentNumber')}
        />
      ),
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('documentNumber')}</span>
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
      accessorKey: 'startCargoOps',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.startCargoOps', { defaultValue: 'Start Cargo Ops' })}
        />
      ),
      cell: ({ row }) => {
        const val = row.getValue<string>('startCargoOps')
        return (
          <span className="text-muted-foreground text-sm">
            {new Date(val).toLocaleTimeString()}
          </span>
        )
      },
    },
    {
      accessorKey: 'endCargoOps',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.endCargoOps', { defaultValue: 'End Cargo Ops' })}
        />
      ),
      cell: ({ row }) => {
        const val = row.getValue<string>('endCargoOps')
        return (
          <span className="text-muted-foreground text-sm">
            {new Date(val).toLocaleTimeString()}
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
