import type { CompanyResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getCompanyColumns } from './companies-columns'

const route = getRouteApi('/_authenticated/catalog/companies/')
const globalFilterFn = createGlobalFilter<CompanyResponse>('commonName', 'legalName')

interface CompaniesTableProps {
  data: CompanyResponse[]
}

export function CompaniesTable({ data }: CompaniesTableProps) {
  return (
    <EntityTable
      tableId="companies"
      data={data}
      getColumns={getCompanyColumns}
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
