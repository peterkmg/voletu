import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { StorageResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell, NumericCell, ResolvedCell, StatusBadge } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { entityActiveColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

export function getStorageColumns(t: TFunction): ColumnDef<StorageResponse>[] {
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
      cell: ({ row }) => <ResolvedCell value={(row.original as any).warehouseIdName} />,
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
      cell: ({ row }) => <ResolvedCell value={(row.original as any).productTypeIdName} />,
    },
    {
      accessorKey: 'isTypeSpecific',
      header: t('catalog:storage.columns.isTypeSpecific'),
      cell: ({ row }) => {
        const value = row.getValue<boolean>('isTypeSpecific')
        return (
          <StatusBadge
            value={value ? 'active' : 'archived'}
            label={value ? t('common:yes') : t('common:no')}
            colorMap={entityActiveColors}
            className="text-xs"
          />
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
      meta: { align: 'right' as const },
      cell: ({ row }) => <DateCell value={row.getValue('createdAt')} />,
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
