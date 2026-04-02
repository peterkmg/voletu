import type { ColumnDef } from '@tanstack/react-table'
import type { AcceptanceResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { useAcceptanceDocumentQuery } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentQuery'
import { getRouteApi } from '@tanstack/react-router'
import { documentStatusColors } from '~/lib/badge-colors'

function getColumns(t: (k: string) => string): ColumnDef<AcceptanceResponse>[] {
  return [
    textColumn('documentNumber', t('common:table.documentNumber'), { primary: true }),
    dateColumn('dateAccepted', t('common:table.date')),
    textColumn('sourceEntity', t('common:table.source')),
    statusColumn('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/incoming/external/')
const globalFilterFn = createGlobalFilter<AcceptanceResponse>('documentNumber')

export function ExternalAcceptancePage() {
  const { t } = useTranslation(['common'])
  const queryResult = useAcceptanceDocumentQuery({
    truckWaybillId: 'isNull' as any,
    railWaybillId: 'isNull' as any,
    transitDispatchId: 'isNull' as any,
  })

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.externalAcceptance')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="external-acceptance"
      />
    </div>
  )
}

export function ExternalAcceptanceDetail() {
  return <div className="p-4">External Acceptance Detail — TODO</div>
}
