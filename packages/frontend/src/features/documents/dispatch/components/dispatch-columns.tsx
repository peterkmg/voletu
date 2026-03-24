import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { dispatchMethodColors, dispatchPurposeColors, documentStatusColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

export function getDispatchColumns(t: TFunction): ColumnDef<DispatchResponse>[] {
  return [
    selectColumn<DispatchResponse>(),
    textColumn<DispatchResponse>('documentNumber', t('documents:dispatch.columns.documentNumber')),
    dateColumn<DispatchResponse>('date', t('documents:dispatch.columns.date')),
    statusColumn<DispatchResponse>('dispatchPurpose', t('documents:dispatch.columns.purpose'), dispatchPurposeColors),
    statusColumn<DispatchResponse>('dispatchMethod', t('documents:dispatch.columns.method'), dispatchMethodColors),
    resolvedColumn<DispatchResponse>('contractorIdName', t('documents:dispatch.columns.contractor', 'Contractor'), 'contractorIdName'),
    resolvedColumn<DispatchResponse>('portIdName', t('documents:dispatch.columns.port', 'Port'), 'portIdName'),
    resolvedColumn<DispatchResponse>('exporterIdName', t('documents:dispatch.columns.exporter', 'Exporter'), 'exporterIdName'),
    resolvedColumn<DispatchResponse>('destinationBaseIdName', t('documents:dispatch.columns.destinationBase', 'Destination Base'), 'destinationBaseIdName'),
    statusColumn<DispatchResponse>('status', t('documents:dispatch.columns.status'), documentStatusColors),
    dateColumn<DispatchResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<DispatchResponse>(DataTableRowActions),
  ]
}
