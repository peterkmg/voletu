import type { StorageResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getStorageColumns } from './storages-columns'

const route = getRouteApi('/_authenticated/catalog/storages/')
const globalFilterFn = createGlobalFilter<StorageResponse>('commonName')

interface StoragesTableProps {
  data: StorageResponse[]
}

export function StoragesTable({ data }: StoragesTableProps) {
  return (
    <EntityTable
      tableId="storages"
      data={data}
      getColumns={getStorageColumns}
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
