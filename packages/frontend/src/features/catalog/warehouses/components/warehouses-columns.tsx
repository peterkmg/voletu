import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { WarehouseResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, LookupCell } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

interface WarehouseColumnLookups {
  baseMap: Map<string, string>
}

export function getWarehouseColumns(t: TFunction, lookups: WarehouseColumnLookups): ColumnDef<WarehouseResponse>[] {
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
      cell: ({ row }) => (
        <LookupCell value={row.getValue('baseId')} lookupMap={lookups.baseMap} />
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
