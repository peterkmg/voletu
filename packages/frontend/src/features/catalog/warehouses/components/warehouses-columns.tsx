import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { WarehouseResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, ResolvedCell } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getWarehouseColumns(t: TFunction): ColumnDef<WarehouseResponse>[] {
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
      accessorKey: 'commonName',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:warehouse.columns.commonName')}
        />
      ),
      meta: { className: 'w-1/3' },
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('commonName')}</span>
      ),
    },
    {
      accessorKey: 'baseId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:warehouse.columns.baseId')}
        />
      ),
      meta: { className: 'w-1/4' },
      cell: ({ row }) => <ResolvedCell value={(row.original as any).baseIdName} />,
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
