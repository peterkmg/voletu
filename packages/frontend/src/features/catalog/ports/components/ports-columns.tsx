import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PortResponse } from '~/generated/types'
import { actionsColumn, dateColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getPortColumns(t: TFunction): ColumnDef<PortResponse>[] {
  return [
    selectColumn<PortResponse>(),
    textColumn<PortResponse>('commonName', t('catalog:port.columns.commonName'), { className: 'w-1/3' }),
    textColumn<PortResponse>('country', t('catalog:port.columns.longName'), { primary: false, className: 'w-1/4' }),
    dateColumn<PortResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<PortResponse>(DataTableRowActions),
  ]
}
