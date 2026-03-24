import type { OwnershipTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getOwnershipTransferColumns } from './ownership-transfer-columns'

const route = getRouteApi('/_authenticated/documents/ownership-transfer/')
const globalFilterFn = createGlobalFilter<OwnershipTransferResponse>('id')

interface OwnershipTransferTableProps {
  data: OwnershipTransferResponse[]
}

export function OwnershipTransferTable({ data }: OwnershipTransferTableProps) {
  return (
    <EntityTable
      data={data}
      getColumns={getOwnershipTransferColumns}
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
