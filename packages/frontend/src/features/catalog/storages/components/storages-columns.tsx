import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { StorageResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, LookupCell, NumericCell } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

interface StorageColumnLookups {
  warehouseMap: Map<string, string>
  productTypeMap: Map<string, string>
}

export function getStorageColumns(t: TFunction, lookups: StorageColumnLookups): ColumnDef<StorageResponse>[] {
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
          title={t('catalog:storage.columns.commonName')}
        />
      ),
      meta: { className: 'w-1/4' },
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('commonName')}</span>
      ),
    },
    {
      accessorKey: 'warehouseId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:storage.columns.warehouseId')}
        />
      ),
      cell: ({ row }) => (
        <LookupCell value={row.getValue('warehouseId')} lookupMap={lookups.warehouseMap} />
      ),
    },
    {
      accessorKey: 'capacity',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:storage.columns.capacity')}
        />
      ),
      meta: { align: 'right' as const },
      cell: ({ row }) => <NumericCell value={row.getValue('capacity')} />,
    },
    {
      accessorKey: 'productTypeId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:storage.columns.productTypeId')}
        />
      ),
      cell: ({ row }) => (
        <LookupCell value={row.getValue('productTypeId')} lookupMap={lookups.productTypeMap} />
      ),
    },
    {
      accessorKey: 'isTypeSpecific',
      header: t('catalog:storage.columns.isTypeSpecific'),
      cell: ({ row }) => {
        const value = row.getValue<boolean>('isTypeSpecific')
        return (
          <Badge variant={value ? 'default' : 'outline'} className="text-xs">
            {value ? t('common:yes') : t('common:no')}
          </Badge>
        )
      },
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
