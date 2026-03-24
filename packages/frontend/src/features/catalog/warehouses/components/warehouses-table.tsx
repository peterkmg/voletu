import type { WarehouseResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getWarehouseColumns } from './warehouses-columns'

const route = getRouteApi('/_authenticated/catalog/warehouses/')
const globalFilterFn = createGlobalFilter<WarehouseResponse>('commonName', 'longName')

interface WarehousesTableProps {
  data: WarehouseResponse[]
}

export function WarehousesTable({ data }: WarehousesTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getWarehouseColumns}
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
