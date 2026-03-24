import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductGroupResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { DataTableRowActions } from './data-table-row-actions'

export function getProductGroupColumns(t: TFunction): ColumnDef<ProductGroupResponse>[] {
  return [
    selectColumn<ProductGroupResponse>(),
    textColumn<ProductGroupResponse>('commonName', t('catalog:productGroup.columns.commonName'), { className: 'w-1/3' }),
    resolvedColumn<ProductGroupResponse>('productTypeId', t('catalog:productGroup.columns.productType'), 'productTypeIdName'),
    dateColumn<ProductGroupResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<ProductGroupResponse>(DataTableRowActions),
  ]
}
