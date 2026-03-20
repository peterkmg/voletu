import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductGroupResponse } from '~/generated/types'
import { DataTableColumnHeader, DateCell } from '~/components/data-table'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getProductGroupColumns(t: TFunction): ColumnDef<ProductGroupResponse>[] {
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
          title={t('catalog:productGroup.columns.commonName')}
        />
      ),
      meta: { className: 'w-1/3' },
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('commonName')}</span>
      ),
    },
    {
      accessorKey: 'productTypeId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:productGroup.columns.productType')}
        />
      ),
      meta: { className: 'w-1/4' },
      cell: ({ row }) => (
        <span className="text-muted-foreground">{row.getValue('productTypeId')}</span>
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
