import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferResponse } from '~/generated/types'
import { actionsColumn, DataTableColumnHeader, dateColumn, DateTimeCell, NumericCell, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { documentStatusColors } from '~/lib/badge-colors'
import { createRowActions } from '~/lib/create-row-actions'
import { usePhysicalTransfer } from './physical-transfer-provider'

const DataTableRowActions = createRowActions<PhysicalTransferResponse>({ useEntity: usePhysicalTransfer, lifecycle: true })

export function getPhysicalTransferColumns(t: TFunction): ColumnDef<PhysicalTransferResponse>[] {
  return [
    selectColumn<PhysicalTransferResponse>(),
    textColumn<PhysicalTransferResponse>('documentNumber', t('documents:acceptance.columns.documentNumber')),
    dateColumn<PhysicalTransferResponse>('date', t('documents:acceptance.columns.date')),
    {
      accessorKey: 'startCargoOps',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.startCargoOps', { defaultValue: 'Start Cargo Ops' })}
        />
      ),
      cell: ({ row }) => <DateTimeCell value={row.getValue('startCargoOps')} />,
    },
    {
      accessorKey: 'endCargoOps',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.endCargoOps', { defaultValue: 'End Cargo Ops' })}
        />
      ),
      cell: ({ row }) => <DateTimeCell value={row.getValue('endCargoOps')} />,
    },
    statusColumn<PhysicalTransferResponse>('status', t('common:table.status'), documentStatusColors),
    {
      id: 'itemsCount',
      header: t('documents:items.title'),
      cell: ({ row }) => (
        <NumericCell value={row.original.items.length} />
      ),
      meta: { align: 'right' as const },
    },
    actionsColumn<PhysicalTransferResponse>(DataTableRowActions),
  ]
}
