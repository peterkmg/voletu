import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductTypeResponse } from '~/generated/types'
import { actionsColumn, dateColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getProductTypeColumns(t: TFunction): ColumnDef<ProductTypeResponse>[] {
  return [
    selectColumn<ProductTypeResponse>(),
    textColumn<ProductTypeResponse>('commonName', t('catalog:productType.columns.commonName'), { className: 'w-1/3' }),
    textColumn<ProductTypeResponse>('longName', t('catalog:productType.columns.longName'), { primary: false, className: 'w-1/4' }),
    dateColumn<ProductTypeResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<ProductTypeResponse>(DataTableRowActions),
  ]
}
