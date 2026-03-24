import type { DispatchResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getDispatchColumns } from './dispatch-columns'

const route = getRouteApi('/_authenticated/documents/dispatch/')
const globalFilterFn = createGlobalFilter<DispatchResponse>('documentNumber')

interface DispatchTableProps {
  data: DispatchResponse[]
}

export function DispatchTable({ data }: DispatchTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getDispatchColumns}
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
