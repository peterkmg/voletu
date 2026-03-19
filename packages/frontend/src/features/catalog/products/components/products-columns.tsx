import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getProductColumns(t: TFunction): ColumnDef<ProductResponse>[] {
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
          title={t('catalog:product.columns.commonName')}
        />
      ),
      meta: { className: 'w-1/4' },
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('commonName')}</span>
      ),
    },
    {
      accessorKey: 'productGroupId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:product.columns.productGroupId')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('productGroupId')}
        </span>
      ),
    },
    {
      accessorKey: 'manufacturerId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:product.columns.manufacturerId')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('manufacturerId') ?? '—'}
        </span>
      ),
    },
    {
      accessorKey: 'identification',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:product.columns.identification')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('identification') ?? '—'}
        </span>
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
