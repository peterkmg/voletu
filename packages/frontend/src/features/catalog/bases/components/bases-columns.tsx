import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BaseResponse } from '~/generated/types/BaseResponse'
import { actionsColumn, dateColumn, selectColumn, textColumn } from '~/components/data-table'
import { createRowActions } from '~/lib/create-row-actions'
import { useBases } from './bases-provider'

const DataTableRowActions = createRowActions<BaseResponse>({ useEntity: useBases })

export function getBaseColumns(t: TFunction): ColumnDef<BaseResponse>[] {
  return [
    selectColumn<BaseResponse>(),
    textColumn<BaseResponse>('commonName', t('catalog:base.columns.commonName')),
    textColumn<BaseResponse>('longName', t('catalog:base.columns.longName'), { primary: false }),
    dateColumn<BaseResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<BaseResponse>(DataTableRowActions),
  ]
}
