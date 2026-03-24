import type { ProductResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getProductColumns } from './products-columns'

const route = getRouteApi('/_authenticated/catalog/products/')
const globalFilterFn = createGlobalFilter<ProductResponse>('commonName', 'addIdentification')

interface ProductsTableProps {
  data: ProductResponse[]
}

export function ProductsTable({ data }: ProductsTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getProductColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      bulkActions={t => [
        {
          label: t('common:actions.softDelete'),
          icon: Archive,
          variant: 'destructive',
          onClick: (rows) => {
            void rows // TODO: wire bulk soft-delete API
          },
        },
      ]}
    />
  )
}
