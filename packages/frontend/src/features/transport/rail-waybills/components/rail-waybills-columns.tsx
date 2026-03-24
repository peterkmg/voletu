import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RailWaybillResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getRailWaybillColumns(t: TFunction): ColumnDef<RailWaybillResponse>[] {
  return [
    selectColumn<RailWaybillResponse>(),
    textColumn<RailWaybillResponse>('documentNumber', t('transport:rail.columns.waybillNumber')),
    dateColumn<RailWaybillResponse>('date', t('transport:rail.columns.date')),
    resolvedColumn<RailWaybillResponse>('senderId', t('transport:rail.columns.sender'), 'senderIdName'),
    dateColumn<RailWaybillResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<RailWaybillResponse>(DataTableRowActions),
  ]
}
