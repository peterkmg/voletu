import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RailReceiptPipelineResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { createEntityProvider } from '~/lib/create-entity-provider'

const pipelineStatusColors: Record<string, string> = {
  PENDING: 'bg-yellow-950 text-yellow-500 border-yellow-800',
  DRAFT: 'bg-blue-950 text-blue-400 border-blue-800',
  EXECUTED: 'bg-green-950 text-green-400 border-green-800',
}

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<RailReceiptPipelineResponse, DialogType>('RailReceipt')

function getColumns(t: TFunction): ColumnDef<RailReceiptPipelineResponse>[] {
  return [
    selectColumn<RailReceiptPipelineResponse>(),
    textColumn<RailReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber')),
    textColumn<RailReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<RailReceiptPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<RailReceiptPipelineResponse>('productName', t('common:table.product')),
    textColumn<RailReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    statusColumn<RailReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    textColumn<RailReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber')),
    textColumn<RailReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
  ]
}

const route = getRouteApi('/_authenticated/incoming/rail/')
const globalFilterFn = createGlobalFilter<RailReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function RailReceiptTable({ data }: { data: RailReceiptPipelineResponse[] }) {
  return (
    <EntityTable
      tableId="rail-receipt"
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

export function RailReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useRailReceiptPipelineQuery()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.railReceipt')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={RailReceiptTable}
      dialogs={Dialogs}
    />
  )
}

export function RailReceiptDetail() {
  return <div className="p-4">Rail Receipt Detail — TODO</div>
}
