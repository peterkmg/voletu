import type { ProductGroupResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getProductGroupColumns } from './product-groups-columns'

const route = getRouteApi('/_authenticated/catalog/product-groups/')
const globalFilterFn = createGlobalFilter<ProductGroupResponse>('commonName')

interface ProductGroupsTableProps {
  data: ProductGroupResponse[]
}

export function ProductGroupsTable({ data }: ProductGroupsTableProps) {
  return (
    <EntityTable
      tableId="product-groups"
      data={data}
      getColumns={getProductGroupColumns}
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
