import type { ColumnDef } from '@tanstack/react-table'
import type { OwnershipTransferResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, statusColumn } from '~/components/data-table'
import { useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { getRouteApi } from '@tanstack/react-router'
import { documentStatusColors } from '~/lib/badge-colors'

function getColumns(t: (k: string) => string): ColumnDef<OwnershipTransferResponse>[] {
  return [
    dateColumn('date', t('common:table.date')),
    statusColumn('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/ownership-transfer/')
const globalFilterFn = createGlobalFilter<OwnershipTransferResponse>('id')

export function OwnershipTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useOwnershipTransferList()

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.ownershipTransfer')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="ownership-transfer"
      />
    </div>
  )
}

export function OwnershipTransferDetail() {
  return <div className="p-4">Ownership Transfer Detail — TODO</div>
}
