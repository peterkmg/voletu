import type { RailWaybillResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getRailWaybillColumns } from './rail-waybills-columns'

const route = getRouteApi('/_authenticated/transport/rail-waybills/')
const globalFilterFn = createGlobalFilter<RailWaybillResponse>('documentNumber')

interface RailWaybillsTableProps {
  data: RailWaybillResponse[]
}

export function RailWaybillsTable({ data }: RailWaybillsTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getRailWaybillColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['transport', 'common']}
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
