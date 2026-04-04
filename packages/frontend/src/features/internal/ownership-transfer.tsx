import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { Button } from '~/components/ui/button'
import { useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity: _useEntity } = createEntityProvider<OwnershipTransferResponse, DialogType>('OwnershipTransfer')

function getColumns(t: TFunction): ColumnDef<OwnershipTransferResponse>[] {
  return [
    selectColumn<OwnershipTransferResponse>(),
    dateColumn<OwnershipTransferResponse>('date', t('common:table.date')),
    statusColumn<OwnershipTransferResponse>('status', t('common:table.status'), documentStatusColors),
  ]
}

const route = getRouteApi('/_authenticated/internal/ownership-transfer/')
const globalFilterFn = createGlobalFilter<OwnershipTransferResponse>('id')

function OwnershipTransferTable({ data }: { data: OwnershipTransferResponse[] }) {
  return (
    <EntityTable
      tableId="ownership-transfer"
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

export function OwnershipTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useOwnershipTransferList()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.ownershipTransfer')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={OwnershipTransferTable}
      dialogs={Dialogs}
    />
  )
}

export function OwnershipTransferDetail() {
  return <div className="p-4">Ownership Transfer Detail — TODO</div>
}
