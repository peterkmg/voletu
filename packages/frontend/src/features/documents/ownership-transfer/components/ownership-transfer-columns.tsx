import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferResponse } from '~/generated/types'
import { actionsColumn, DataTableColumnHeader, dateColumn, IdCell, NumericCell, selectColumn, statusColumn } from '~/components/data-table'
import { documentStatusColors } from '~/lib/badge-colors'
import { createRowActions } from '~/lib/create-row-actions'
import { useOwnershipTransfer } from './ownership-transfer-provider'

const DataTableRowActions = createRowActions<OwnershipTransferResponse>({ useEntity: useOwnershipTransfer, lifecycle: true })

export function getOwnershipTransferColumns(t: TFunction): ColumnDef<OwnershipTransferResponse>[] {
  return [
    selectColumn<OwnershipTransferResponse>(),
    {
      accessorKey: 'id',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.id')}
        />
      ),
      cell: ({ row }) => <IdCell value={row.getValue('id')} />,
    },
    dateColumn<OwnershipTransferResponse>('date', t('documents:acceptance.columns.date')),
    statusColumn<OwnershipTransferResponse>('status', t('common:table.status'), documentStatusColors),
    {
      id: 'itemsCount',
      header: t('documents:items.title'),
      cell: ({ row }) => (
        <NumericCell value={row.original.items.length} />
      ),
      meta: { align: 'right' as const },
    },
    actionsColumn<OwnershipTransferResponse>(DataTableRowActions),
  ]
}
