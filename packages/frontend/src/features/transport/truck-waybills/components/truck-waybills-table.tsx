import type { TruckWaybillResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getTruckWaybillColumns } from './truck-waybills-columns'

const route = getRouteApi('/_authenticated/transport/truck-waybills/')
const globalFilterFn = createGlobalFilter<TruckWaybillResponse>('documentNumber')

interface TruckWaybillsTableProps {
  data: TruckWaybillResponse[]
}

export function TruckWaybillsTable({ data }: TruckWaybillsTableProps) {
  return (
    <EntityTable
      tableId="truck-waybills"
      data={data}
      getColumns={getTruckWaybillColumns}
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
