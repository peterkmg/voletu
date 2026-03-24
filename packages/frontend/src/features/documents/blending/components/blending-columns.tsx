import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { documentStatusColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

export function getBlendingColumns(t: TFunction): ColumnDef<BlendingResponse>[] {
  return [
    selectColumn<BlendingResponse>(),
    textColumn<BlendingResponse>('documentNumber', t('documents:blending.columns.documentNumber')),
    dateColumn<BlendingResponse>('date', t('documents:blending.columns.date')),
    resolvedColumn<BlendingResponse>('contractorId', t('documents:items.contractor'), 'contractorIdName'),
    resolvedColumn<BlendingResponse>('targetProductId', t('documents:items.product'), 'targetProductIdName'),
    statusColumn<BlendingResponse>('status', t('documents:blending.columns.status'), documentStatusColors),
    dateColumn<BlendingResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<BlendingResponse>(DataTableRowActions),
  ]
}
