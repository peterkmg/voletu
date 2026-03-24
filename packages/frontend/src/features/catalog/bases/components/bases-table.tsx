import type { BaseResponse } from '~/generated/types/BaseResponse'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getBaseColumns } from './bases-columns'

const route = getRouteApi('/_authenticated/catalog/bases/')
const globalFilterFn = createGlobalFilter<BaseResponse>('commonName', 'longName')

interface BasesTableProps {
  data: BaseResponse[]
}

export function BasesTable({ data }: BasesTableProps) {
  return (
    <EntityTable
      tableId="bases"
      data={data}
      getColumns={getBaseColumns}
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
