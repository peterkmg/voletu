import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchFlatRow, DispatchItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { dispatchDocumentExecute, dispatchDocumentHardDelete, dispatchDocumentRevert, dispatchDocumentSoftDelete } from '~/generated/client'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { flowDispatchFlatQueryQueryKey, useFlowDispatchFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowDispatchFlatQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { DirectDispatchMutateDialog } from './direct-dispatch/direct-dispatch-mutate-dialog'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<DispatchFlatRow> }>,
): ColumnDef<DispatchFlatRow>[] {
  return [

    { ...textColumn<DispatchFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<DispatchFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<DispatchFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },

    { ...textColumn<DispatchFlatRow>('productIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<DispatchFlatRow>('storageIdName', t('common:columns.storage'), { primary: false }), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<DispatchFlatRow>('dispatchedAmount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<DispatchFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },

    { ...actionsColumn<DispatchFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/outgoing/direct/')
const detailRoute = getRouteApi('/_authenticated/outgoing/direct/$id')
const globalFilterFn = createGlobalFilter<DispatchFlatRow>('documentNumber', 'productIdName', 'storageIdName')

function DirectDispatchTable({
  data,
  actions,
  RowActions,
}: {
  data: DispatchFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<DispatchFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="direct-dispatch"
      data={data}
      getColumns={t => getColumns(t, RowActions)}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
      actions={actions}
    />
  )
}

function useDirectDispatchTitle() {
  return useTranslation(['common', 'documents']).t('common:nav.directDispatch')
}

function useDirectDispatchText() {
  const title = useDirectDispatchTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('documents:document.directDispatch'),
  }
}

interface DirectDispatchDetailData {
  id: string
  documentNumber: string
  status: string
  date: string
  contractorId?: string | null
  contractorIdName?: string | null
  items: DispatchItemResponse[]
  executedAt?: string | null
}

const directDispatchViewDefinition = defineDocumentViews<DispatchFlatRow, DirectDispatchDetailData>({
  displayName: 'DirectDispatch',
  useText: useDirectDispatchText,
  useQuery: () => useFlowDispatchFlatQuery({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
  Table: DirectDispatchTable,
  MutateDialog: DirectDispatchMutateDialog,
  deleteDialog: {
    hardDeleteFn: dispatchDocumentHardDelete,
    softDeleteFn: dispatchDocumentSoftDelete,
    queryKey: () => flowDispatchFlatQueryQueryKey({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
    entityLabel: 'common:nav.directDispatch',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: dispatchDocumentExecute,
    revertFn: dispatchDocumentRevert,
    queryKey: () => flowDispatchFlatQueryQueryKey({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
  },
  rowActions: {
    getDetailPath: row => `/outgoing/direct/${row.documentId}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: id => useDispatchCompositeGet(id, { embed: 'names' }),
    backTo: '/outgoing/direct',
    statusColorMap: statusColors,
    getDocument: data => ({
      id: data.id,
      documentNumber: data.documentNumber,
      status: data.status,
    }),
    renderFormContent: ({ data, t }) => {
      return (
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(data.date)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{data.contractorIdName ?? data.contractorId}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <ChildItemsTable
          items={data.items}
          columns={[
            textColumn<DispatchItemResponse>('productIdName', t('common:table.product')),
            textColumn<DispatchItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<DispatchItemResponse>('dispatchedAmount', t('common:table.quantity')),
          ]}
          isLocked={isLocked}
          sectionTitle={t('direct-dispatch:section.items')}
        />
      )
    },
    renderMetadataContent: ({ data, t }) => {
      if (!data.executedAt) {
        return null
      }
      return (
        <div className="text-sm">
          <span className="text-muted-foreground">
            {t('documents:metadata.executedAt')}
            :
          </span>
          {' '}
          {formatDateTime(data.executedAt)}
        </div>
      )
    },
  },
})

export function DirectDispatchPage() {
  return <directDispatchViewDefinition.View />
}

export function DirectDispatchDetail() {
  return <directDispatchViewDefinition.DetailView />
}
