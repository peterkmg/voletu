import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckDispatchPipelineResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { createEntityProvider } from '~/lib/create-entity-provider'

const pipelineStatusColors: Record<string, string> = {
  DRAFT: 'bg-blue-950 text-blue-400 border-blue-800',
  EXECUTED: 'bg-green-950 text-green-400 border-green-800',
}

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<TruckDispatchPipelineResponse, DialogType>('TruckDispatch')

function getColumns(t: TFunction): ColumnDef<TruckDispatchPipelineResponse>[] {
  return [
    selectColumn<TruckDispatchPipelineResponse>(),
    textColumn<TruckDispatchPipelineResponse>('documentNumber', t('common:table.documentNumber')),
    textColumn<TruckDispatchPipelineResponse>('date', t('common:table.date')),
    textColumn<TruckDispatchPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<TruckDispatchPipelineResponse>('productName', t('common:table.product')),
    textColumn<TruckDispatchPipelineResponse>('dispatchedQuantity', t('common:table.quantity')),
    statusColumn<TruckDispatchPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/outgoing/truck/')
const globalFilterFn = createGlobalFilter<TruckDispatchPipelineResponse>('documentNumber', 'contractorName')

function TruckDispatchTable({ data }: { data: TruckDispatchPipelineResponse[] }) {
  return (
    <EntityTable
      tableId="truck-dispatch"
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

export function TruckDispatchPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useTruckDispatchPipelineQuery()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.truckDispatch')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={TruckDispatchTable}
      dialogs={Dialogs}
    />
  )
}

export function TruckDispatchDetail() {
  return <div className="p-4">Truck Dispatch Detail — TODO</div>
}
