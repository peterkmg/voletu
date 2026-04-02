import type { ColumnDef } from '@tanstack/react-table'
import type { TruckReceiptPipelineResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { getRouteApi } from '@tanstack/react-router'

const pipelineStatusColors: Record<string, string> = {
  PENDING: 'bg-yellow-950 text-yellow-500 border-yellow-800',
  DRAFT: 'bg-blue-950 text-blue-400 border-blue-800',
  EXECUTED: 'bg-green-950 text-green-400 border-green-800',
}

function getColumns(t: (k: string) => string): ColumnDef<TruckReceiptPipelineResponse>[] {
  return [
    textColumn('basisDocumentNumber', t('common:table.waybillNumber'), { primary: true }),
    textColumn('basisDate', t('common:table.date')),
    textColumn('contractorName', t('common:table.contractor')),
    textColumn('productName', t('common:table.product')),
    textColumn('expectedQuantity', t('common:table.expectedQty')),
    statusColumn('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    textColumn('actionDocumentNumber', t('common:table.acceptanceNumber')),
    textColumn('actualQuantity', t('common:table.actualQty')),
  ]
}

const route = getRouteApi('/_authenticated/incoming/truck/')
const globalFilterFn = createGlobalFilter<TruckReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

export function TruckReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowTruckReceiptQuery()

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.truckReceipt')}</h1>
      <EntityTable
        data={queryResult.data?.data ?? []}
        getColumns={getColumns}
        routeApi={route}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        isLoading={queryResult.isLoading}
        tableId="truck-receipt"
      />
    </div>
  )
}

export function TruckReceiptDetail() {
  return <div className="p-4">Truck Receipt Detail — TODO</div>
}
