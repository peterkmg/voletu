import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useBlendingDocumentList } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<BlendingResponse, DialogType>('Blending')

function getColumns(t: TFunction): ColumnDef<BlendingResponse>[] {
  return [
    selectColumn<BlendingResponse>(),
    textColumn<BlendingResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<BlendingResponse>('date', t('common:table.date')),
    resolvedColumn<BlendingResponse>('contractorId', t('common:table.contractor'), 'contractorIdName'),
    resolvedColumn<BlendingResponse>('targetProductId', t('common:table.product'), 'targetProductIdName'),
    statusColumn<BlendingResponse>('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/blending/')
const globalFilterFn = createGlobalFilter<BlendingResponse>('documentNumber')

function BlendingTable({ data }: { data: BlendingResponse[] }) {
  return (
    <EntityTable
      tableId="blending-internal"
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

export function BlendingPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useBlendingDocumentList()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.blending')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={BlendingTable}
      dialogs={Dialogs}
    />
  )
}

export function BlendingDetail() {
  return <div className="p-4">Blending Detail — TODO</div>
}
