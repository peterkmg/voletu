import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useDispatchDocumentQuery } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentQuery'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<DispatchResponse, DialogType>('Bunkering')

function getColumns(t: TFunction): ColumnDef<DispatchResponse>[] {
  return [
    selectColumn<DispatchResponse>(),
    textColumn<DispatchResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<DispatchResponse>('date', t('common:table.date')),
    resolvedColumn<DispatchResponse>('contractorId', t('common:table.contractor'), 'contractorIdName'),
    statusColumn<DispatchResponse>('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/outgoing/bunkering/')
const globalFilterFn = createGlobalFilter<DispatchResponse>('documentNumber')

function BunkeringTable({ data }: { data: DispatchResponse[] }) {
  return (
    <EntityTable
      tableId="bunkering"
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

export function BunkeringPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useDispatchDocumentQuery({
    dispatchMethod: 'BUNKERING' as any,
  })

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.bunkering')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={BunkeringTable}
      dialogs={Dialogs}
    />
  )
}

export function BunkeringDetail() {
  return <div className="p-4">Bunkering Detail — TODO</div>
}
