import type { AcceptanceResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getAcceptanceColumns } from './acceptance-columns'

const route = getRouteApi('/_authenticated/documents/acceptance/')
const globalFilterFn = createGlobalFilter<AcceptanceResponse>('documentNumber', 'sourceEntity')

interface AcceptanceTableProps {
  data: AcceptanceResponse[]
}

export function AcceptanceTable({ data }: AcceptanceTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getAcceptanceColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['documents', 'common']}
      bulkActions={t => [
        {
          label: t('common:actions.execute'),
          icon: Play,
          onClick: (rows) => {
            const draftRows = rows.filter(r => r.status === 'DRAFT')
            void draftRows // TODO: wire bulk execute API
          },
        },
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
