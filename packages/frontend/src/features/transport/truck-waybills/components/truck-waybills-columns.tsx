import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckWaybillResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getTruckWaybillColumns(t: TFunction): ColumnDef<TruckWaybillResponse>[] {
  return [
    selectColumn<TruckWaybillResponse>(),
    textColumn<TruckWaybillResponse>('documentNumber', t('transport:truck.columns.waybillNumber')),
    dateColumn<TruckWaybillResponse>('date', t('transport:truck.columns.date')),
    resolvedColumn<TruckWaybillResponse>('senderId', t('transport:truck.columns.sender'), 'senderIdName'),
    dateColumn<TruckWaybillResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<TruckWaybillResponse>(DataTableRowActions),
  ]
}
