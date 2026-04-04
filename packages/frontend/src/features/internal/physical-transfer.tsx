import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<PhysicalTransferResponse, DialogType>('PhysicalTransfer')

function getColumns(t: TFunction): ColumnDef<PhysicalTransferResponse>[] {
  return [
    selectColumn<PhysicalTransferResponse>(),
    textColumn<PhysicalTransferResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<PhysicalTransferResponse>('date', t('common:table.date')),
    statusColumn<PhysicalTransferResponse>('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/physical-transfer/')
const globalFilterFn = createGlobalFilter<PhysicalTransferResponse>('documentNumber')

function PhysicalTransferTable({ data }: { data: PhysicalTransferResponse[] }) {
  return (
    <EntityTable
      tableId="physical-transfer"
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

export function PhysicalTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = usePhysicalTransferList()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.physicalTransfer')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={PhysicalTransferTable}
      dialogs={Dialogs}
    />
  )
}

export function PhysicalTransferDetail() {
  return <div className="p-4">Physical Transfer Detail — TODO</div>
}
