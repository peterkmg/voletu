import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { InventoryReconciliationResponse } from '~/generated/types'
import { actionsColumn, DataTableColumnHeader, dateColumn, LookupCell, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { documentStatusColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

interface ReconciliationColumnLookups {
  warehouseMap: Map<string, string>
}

export function getReconciliationColumns(t: TFunction, lookups: ReconciliationColumnLookups): ColumnDef<InventoryReconciliationResponse>[] {
  return [
    selectColumn<InventoryReconciliationResponse>(),
    textColumn<InventoryReconciliationResponse>('documentNumber', t('documents:reconciliation.columns.documentNumber')),
    dateColumn<InventoryReconciliationResponse>('date', t('documents:reconciliation.columns.date')),
    {
      accessorKey: 'warehouseId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:nav.warehouses')}
        />
      ),
      cell: ({ row }) => (
        <LookupCell value={row.getValue('warehouseId')} lookupMap={lookups.warehouseMap} />
      ),
    },
    statusColumn<InventoryReconciliationResponse>('status', t('documents:reconciliation.columns.status'), documentStatusColors),
    dateColumn<InventoryReconciliationResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<InventoryReconciliationResponse>(DataTableRowActions),
  ]
}
