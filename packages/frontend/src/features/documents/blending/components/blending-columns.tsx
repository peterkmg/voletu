import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, ResolvedCell, StatusBadge } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { documentStatusColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

export function getBlendingColumns(t: TFunction): ColumnDef<BlendingResponse>[] {
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
          title={t('documents:blending.columns.documentNumber')}
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
          title={t('documents:blending.columns.date')}
        />
      ),
      meta: { align: 'right' as const },
      cell: ({ row }) => <DateCell value={row.getValue('date')} />,
    },
    {
      accessorKey: 'contractorId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:items.contractor')}
        />
      ),
      cell: ({ row }) => <ResolvedCell value={(row.original as any).contractorIdName} />,
    },
    {
      accessorKey: 'targetProductId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:items.product')}
        />
      ),
      cell: ({ row }) => <ResolvedCell value={(row.original as any).targetProductIdName} />,
    },
    {
      accessorKey: 'status',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:blending.columns.status')}
        />
      ),
      cell: ({ row }) => (
        <StatusBadge value={row.getValue('status')} colorMap={documentStatusColors} />
      ),
    },
    {
      accessorKey: 'createdAt',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.createdAt')}
        />
      ),
      meta: { align: 'right' as const },
      cell: ({ row }) => <DateCell value={row.getValue('createdAt')} />,
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
