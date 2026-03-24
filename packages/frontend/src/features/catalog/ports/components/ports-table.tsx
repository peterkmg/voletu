import type { PortResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getPortColumns } from './ports-columns'

const route = getRouteApi('/_authenticated/catalog/ports/')
const globalFilterFn = createGlobalFilter<PortResponse>('commonName', 'country')

interface PortsTableProps {
  data: PortResponse[]
}

export function PortsTable({ data }: PortsTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getPortColumns}
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
