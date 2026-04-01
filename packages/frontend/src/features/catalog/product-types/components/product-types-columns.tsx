import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductTypeResponse } from '~/generated/types'
import { actionsColumn, dateColumn, selectColumn, textColumn } from '~/components/data-table'
import { createRowActions } from '~/lib/create-row-actions'
import { useProductTypes } from './product-types-provider'

const DataTableRowActions = createRowActions<ProductTypeResponse>({ useEntity: useProductTypes })

export function getProductTypeColumns(t: TFunction): ColumnDef<ProductTypeResponse>[] {
  return [
    selectColumn<ProductTypeResponse>(),
    textColumn<ProductTypeResponse>('commonName', t('catalog:productType.columns.commonName')),
    textColumn<ProductTypeResponse>('longName', t('catalog:productType.columns.longName'), { primary: false }),
    dateColumn<ProductTypeResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<ProductTypeResponse>(DataTableRowActions),
  ]
}
