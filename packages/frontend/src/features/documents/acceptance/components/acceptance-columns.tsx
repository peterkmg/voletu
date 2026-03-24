import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceResponse } from '~/generated/types'
import { actionsColumn, dateColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { arrivalTypeColors, documentStatusColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

export function getAcceptanceColumns(t: TFunction): ColumnDef<AcceptanceResponse>[] {
  return [
    selectColumn<AcceptanceResponse>(),
    textColumn<AcceptanceResponse>('documentNumber', t('documents:acceptance.columns.documentNumber')),
    dateColumn<AcceptanceResponse>('dateAccepted', t('documents:acceptance.columns.date')),
    statusColumn<AcceptanceResponse>('arrivalType', t('documents:acceptance.columns.arrivalType'), arrivalTypeColors),
    statusColumn<AcceptanceResponse>('status', t('documents:acceptance.columns.status'), documentStatusColors),
    dateColumn<AcceptanceResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<AcceptanceResponse>(DataTableRowActions),
  ]
}
