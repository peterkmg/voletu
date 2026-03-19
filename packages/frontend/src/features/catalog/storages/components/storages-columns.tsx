import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { StorageResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { Checkbox } from '~/components/ui/checkbox'
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
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('warehouseId')}
        </span>
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
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('capacity') ?? '\u2014'}
        </span>
      ),
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
        <span className="text-muted-foreground">
          {row.getValue('productTypeId') ?? '\u2014'}
        </span>
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
      cell: ({ row }) => {
        const date = row.getValue<string>('createdAt')
        return (
          <span className="text-muted-foreground text-sm">
            {new Date(date).toLocaleDateString()}
          </span>
        )
      },
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
