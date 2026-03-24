import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BaseResponse } from '~/generated/types/BaseResponse'
import { actionsColumn, dateColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getBaseColumns(t: TFunction): ColumnDef<BaseResponse>[] {
  return [
    selectColumn<BaseResponse>(),
    textColumn<BaseResponse>('commonName', t('catalog:base.columns.commonName'), { className: 'w-1/3' }),
    textColumn<BaseResponse>('longName', t('catalog:base.columns.longName'), { primary: false, className: 'w-1/4' }),
    dateColumn<BaseResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<BaseResponse>(DataTableRowActions),
  ]
}
