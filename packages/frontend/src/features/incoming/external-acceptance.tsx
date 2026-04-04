import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useAcceptanceDocumentQuery } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentQuery'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<AcceptanceResponse, DialogType>('ExternalAcceptance')

function getColumns(t: TFunction): ColumnDef<AcceptanceResponse>[] {
  return [
    selectColumn<AcceptanceResponse>(),
    textColumn<AcceptanceResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<AcceptanceResponse>('dateAccepted', t('common:table.date')),
    textColumn<AcceptanceResponse>('sourceEntity', t('common:table.source')),
    statusColumn<AcceptanceResponse>('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/incoming/external/')
const globalFilterFn = createGlobalFilter<AcceptanceResponse>('documentNumber')

function ExternalAcceptanceTable({ data }: { data: AcceptanceResponse[] }) {
  return (
    <EntityTable
      tableId="external-acceptance"
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

export function ExternalAcceptancePage() {
  const { t } = useTranslation(['common'])
  const queryResult = useAcceptanceDocumentQuery({
    truckWaybillId: 'isNull' as any,
    railWaybillId: 'isNull' as any,
    transitDispatchId: 'isNull' as any,
  })

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.externalAcceptance')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={ExternalAcceptanceTable}
      dialogs={Dialogs}
    />
  )
}

export function ExternalAcceptanceDetail() {
  return <div className="p-4">External Acceptance Detail — TODO</div>
}
