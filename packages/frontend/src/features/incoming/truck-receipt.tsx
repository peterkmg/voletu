import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckReceiptPipelineResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { createEntityProvider } from '~/lib/create-entity-provider'

const pipelineStatusColors: Record<string, string> = {
  PENDING: 'bg-yellow-950 text-yellow-500 border-yellow-800',
  DRAFT: 'bg-blue-950 text-blue-400 border-blue-800',
  EXECUTED: 'bg-green-950 text-green-400 border-green-800',
}

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<TruckReceiptPipelineResponse, DialogType>('TruckReceipt')

function getColumns(t: TFunction): ColumnDef<TruckReceiptPipelineResponse>[] {
  return [
    selectColumn<TruckReceiptPipelineResponse>(),
    textColumn<TruckReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber')),
    textColumn<TruckReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<TruckReceiptPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<TruckReceiptPipelineResponse>('productName', t('common:table.product')),
    textColumn<TruckReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    statusColumn<TruckReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    textColumn<TruckReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber')),
    textColumn<TruckReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
  ]
}

const route = getRouteApi('/_authenticated/incoming/truck/')
const globalFilterFn = createGlobalFilter<TruckReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function TruckReceiptTable({ data }: { data: TruckReceiptPipelineResponse[] }) {
  return (
    <EntityTable
      tableId="truck-receipt"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
    />
  )
}

function PrimaryButtons() {
  const { t } = useTranslation('common')
  return (
    <Button size="sm">
      <Plus className="mr-1 size-4" />
      {t('actions.create')}
    </Button>
  )
}

function Dialogs() {
  return null
}

export function TruckReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowTruckReceiptQuery()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.truckReceipt')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={TruckReceiptTable}
      dialogs={Dialogs}
    />
  )
}

export function TruckReceiptDetail() {
  return <div className="p-4">Truck Receipt Detail — TODO</div>
}
