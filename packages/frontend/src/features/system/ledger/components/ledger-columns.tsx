import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { LedgerEntryResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'

export function getLedgerColumns(t: TFunction): ColumnDef<LedgerEntryResponse>[] {
  return [
    {
      accessorKey: 'storageId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.storage')}
        />
      ),
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('storageId')}</span>
      ),
    },
    {
      accessorKey: 'productId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.product')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('productId')}
        </span>
      ),
    },
    {
      accessorKey: 'contractorId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.contractor')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('contractorId')}
        </span>
      ),
    },
    {
      accessorKey: 'currentAmount',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.quantity')}
        />
      ),
      cell: ({ row }) => (
        <span className="font-medium">
          {row.getValue('currentAmount')}
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
  ]
}
