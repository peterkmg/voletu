import type { ProductTypeResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getProductTypeColumns } from './product-types-columns'

const route = getRouteApi('/_authenticated/catalog/product-types/')
const globalFilterFn = createGlobalFilter<ProductTypeResponse>('commonName', 'longName')

interface ProductTypesTableProps {
  data: ProductTypeResponse[]
}

export function ProductTypesTable({ data }: ProductTypesTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getProductTypeColumns}
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
