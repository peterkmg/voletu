import type { ColumnDef } from '@tanstack/react-table'
import type { TruckDispatchPipelineResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { getRouteApi } from '@tanstack/react-router'

const pipelineStatusColors: Record<string, string> = {
  DRAFT: 'bg-blue-950 text-blue-400 border-blue-800',
  EXECUTED: 'bg-green-950 text-green-400 border-green-800',
}

function getColumns(t: (k: string) => string): ColumnDef<TruckDispatchPipelineResponse>[] {
  return [
    textColumn('documentNumber', t('common:table.documentNumber'), { primary: true }),
    textColumn('date', t('common:table.date')),
    textColumn('contractorName', t('common:table.contractor')),
    textColumn('productName', t('common:table.product')),
    textColumn('dispatchedQuantity', t('common:table.quantity')),
    statusColumn('pipelineStatus', t('common:table.status'), pipelineStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/outgoing/truck/')
const globalFilterFn = createGlobalFilter<TruckDispatchPipelineResponse>('documentNumber', 'contractorName')

export function TruckDispatchPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useTruckDispatchPipelineQuery()

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.truckDispatch')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="truck-dispatch"
      />
    </div>
  )
}

export function TruckDispatchDetail() {
  return <div className="p-4">Truck Dispatch Detail — TODO</div>
}
