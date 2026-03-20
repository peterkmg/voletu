import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, IdCell, NumericCell, StatusBadge } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { documentStatusColors } from '~/lib/badge-colors'
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
      cell: ({ row }) => <IdCell value={row.getValue('id')} />,
    },
    {
      accessorKey: 'date',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:acceptance.columns.date')}
        />
      ),
      meta: { align: 'right' as const },
      cell: ({ row }) => <DateCell value={row.getValue('date')} />,
    },
    {
      accessorKey: 'status',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.status')}
        />
      ),
      cell: ({ row }) => (
        <StatusBadge value={row.getValue('status')} colorMap={documentStatusColors} />
      ),
    },
    {
      id: 'itemsCount',
      header: t('documents:items.title'),
      cell: ({ row }) => (
        <NumericCell value={row.original.items.length} />
      ),
      meta: { align: 'right' as const },
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
