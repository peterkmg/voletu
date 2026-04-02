import type { ColumnDef } from '@tanstack/react-table'
import type { DispatchResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, resolvedColumn, statusColumn, textColumn } from '~/components/data-table'
import { useDispatchDocumentQuery } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentQuery'
import { getRouteApi } from '@tanstack/react-router'
import { documentStatusColors } from '~/lib/badge-colors'

function getColumns(t: (k: string) => string): ColumnDef<DispatchResponse>[] {
  return [
    textColumn('documentNumber', t('common:table.documentNumber'), { primary: true }),
    dateColumn('date', t('common:table.date')),
    resolvedColumn('contractorId', t('common:table.contractor'), 'contractorIdName'),
    statusColumn('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/outgoing/bunkering/')
const globalFilterFn = createGlobalFilter<DispatchResponse>('documentNumber')

export function BunkeringPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useDispatchDocumentQuery({
    dispatchMethod: 'BUNKERING' as any,
  })

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.bunkering')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="bunkering"
      />
    </div>
  )
}

export function BunkeringDetail() {
  return <div className="p-4">Bunkering Detail — TODO</div>
}
