import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductResponse } from '~/generated/types'
import { actionsColumn, dateColumn, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { createRowActions } from '~/lib/create-row-actions'
import { useProducts } from './products-provider'

const DataTableRowActions = createRowActions<ProductResponse>({ useEntity: useProducts })

export function getProductColumns(t: TFunction): ColumnDef<ProductResponse>[] {
  return [
    selectColumn<ProductResponse>(),
    textColumn<ProductResponse>('commonName', t('catalog:product.columns.commonName')),
    resolvedColumn<ProductResponse>('productGroupId', t('catalog:product.columns.productGroupId'), 'productGroupIdName'),
    resolvedColumn<ProductResponse>('manufacturerId', t('catalog:product.columns.manufacturerId'), 'manufacturerIdName'),
    textColumn<ProductResponse>('addIdentification', t('catalog:product.columns.identification'), { primary: false }),
    dateColumn<ProductResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<ProductResponse>(DataTableRowActions),
  ]
}
