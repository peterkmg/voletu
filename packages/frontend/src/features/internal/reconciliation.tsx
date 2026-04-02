import type { ColumnDef } from '@tanstack/react-table'
import type { InventoryReconciliationResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { useReconciliationList } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { getRouteApi } from '@tanstack/react-router'
import { documentStatusColors } from '~/lib/badge-colors'

function getColumns(t: (k: string) => string): ColumnDef<InventoryReconciliationResponse>[] {
  return [
    textColumn('documentNumber', t('common:table.documentNumber'), { primary: true }),
    dateColumn('date', t('common:table.date')),
    statusColumn('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/reconciliation/')
const globalFilterFn = createGlobalFilter<InventoryReconciliationResponse>('documentNumber')

export function ReconciliationPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useReconciliationList()

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.reconciliation')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="reconciliation"
      />
    </div>
  )
}

export function ReconciliationDetail() {
  return <div className="p-4">Reconciliation Detail — TODO</div>
}
