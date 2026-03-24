import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { WarehouseResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getWarehouseColumns(t: TFunction): ColumnDef<WarehouseResponse>[] {
  return [
    selectColumn<WarehouseResponse>(),
    textColumn<WarehouseResponse>('commonName', t('catalog:warehouse.columns.commonName'), { className: 'w-1/3' }),
    resolvedColumn<WarehouseResponse>('baseId', t('catalog:warehouse.columns.baseId'), 'baseIdName'),
    dateColumn<WarehouseResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<WarehouseResponse>(DataTableRowActions),
  ]
}
