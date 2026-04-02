import type { ColumnDef } from '@tanstack/react-table'
import type { BlendingResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, resolvedColumn, statusColumn, textColumn } from '~/components/data-table'
import { useBlendingDocumentList } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { getRouteApi } from '@tanstack/react-router'
import { documentStatusColors } from '~/lib/badge-colors'

function getColumns(t: (k: string) => string): ColumnDef<BlendingResponse>[] {
  return [
    textColumn('documentNumber', t('common:table.documentNumber'), { primary: true }),
    dateColumn('date', t('common:table.date')),
    resolvedColumn('contractorId', t('common:table.contractor'), 'contractorIdName'),
    resolvedColumn('targetProductId', t('common:table.product'), 'targetProductIdName'),
    statusColumn('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/blending/')
const globalFilterFn = createGlobalFilter<BlendingResponse>('documentNumber')

export function BlendingPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useBlendingDocumentList()

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.blending')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="blending-internal"
      />
    </div>
  )
}

export function BlendingDetail() {
  return <div className="p-4">Blending Detail — TODO</div>
}
