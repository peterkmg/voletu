import type { BlendingResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getBlendingColumns } from './blending-columns'

const route = getRouteApi('/_authenticated/documents/blending/')
const globalFilterFn = createGlobalFilter<BlendingResponse>('documentNumber')

interface BlendingTableProps {
  data: BlendingResponse[]
}

export function BlendingTable({ data }: BlendingTableProps) {
  return (
    <EntityTable
      tableId="blending"
      data={data}
      getColumns={getBlendingColumns}
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
