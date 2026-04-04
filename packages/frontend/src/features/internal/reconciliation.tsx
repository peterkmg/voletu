import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { InventoryReconciliationResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useReconciliationList } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<InventoryReconciliationResponse, DialogType>('Reconciliation')

function getColumns(t: TFunction): ColumnDef<InventoryReconciliationResponse>[] {
  return [
    selectColumn<InventoryReconciliationResponse>(),
    textColumn<InventoryReconciliationResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<InventoryReconciliationResponse>('date', t('common:table.date')),
    statusColumn<InventoryReconciliationResponse>('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/reconciliation/')
const globalFilterFn = createGlobalFilter<InventoryReconciliationResponse>('documentNumber')

function ReconciliationTable({ data }: { data: InventoryReconciliationResponse[] }) {
  return (
    <EntityTable
      tableId="reconciliation"
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

export function ReconciliationPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useReconciliationList()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.reconciliation')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={ReconciliationTable}
      dialogs={Dialogs}
    />
  )
}

export function ReconciliationDetail() {
  return <div className="p-4">Reconciliation Detail — TODO</div>
}
