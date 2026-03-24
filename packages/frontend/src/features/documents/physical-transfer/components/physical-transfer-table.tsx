import type { PhysicalTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getPhysicalTransferColumns } from './physical-transfer-columns'

const route = getRouteApi('/_authenticated/documents/physical-transfer/')
const globalFilterFn = createGlobalFilter<PhysicalTransferResponse>('documentNumber')

interface PhysicalTransferTableProps {
  data: PhysicalTransferResponse[]
}

export function PhysicalTransferTable({ data }: PhysicalTransferTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getPhysicalTransferColumns}
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
