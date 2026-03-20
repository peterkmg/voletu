import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, LookupCell, StatusBadge } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { documentStatusColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

interface BlendingColumnLookups {
  companyMap: Map<string, string>
  productMap: Map<string, string>
}

export function getBlendingColumns(t: TFunction, lookups: BlendingColumnLookups): ColumnDef<BlendingResponse>[] {
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
      cell: ({ row }) => (
        <LookupCell value={row.getValue('contractorId')} lookupMap={lookups.companyMap} />
      ),
    },
    {
      accessorKey: 'targetProductId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:items.product')}
        />
      ),
      cell: ({ row }) => (
        <LookupCell value={row.getValue('targetProductId')} lookupMap={lookups.productMap} />
      ),
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
      cell: ({ row }) => <DateCell value={row.getValue('createdAt')} />,
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
