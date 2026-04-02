import type { ColumnDef } from '@tanstack/react-table'
import type { PhysicalTransferResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { getRouteApi } from '@tanstack/react-router'
import { documentStatusColors } from '~/lib/badge-colors'

function getColumns(t: (k: string) => string): ColumnDef<PhysicalTransferResponse>[] {
  return [
    textColumn('documentNumber', t('common:table.documentNumber'), { primary: true }),
    dateColumn('date', t('common:table.date')),
    statusColumn('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/physical-transfer/')
const globalFilterFn = createGlobalFilter<PhysicalTransferResponse>('documentNumber')

export function PhysicalTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = usePhysicalTransferList()

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.physicalTransfer')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="physical-transfer"
      />
    </div>
  )
}

export function PhysicalTransferDetail() {
  return <div className="p-4">Physical Transfer Detail — TODO</div>
}
