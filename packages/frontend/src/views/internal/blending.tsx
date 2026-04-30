import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingComponentResponse, BlendingFlatRow, BlendingResultResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { blendingDocumentExecute, blendingDocumentHardDelete, blendingDocumentRevert, blendingDocumentSoftDelete } from '~/generated/client'
import { useBlendingCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useBlendingCompositeGet'
import { flowBlendingFlatQueryQueryKey, useFlowBlendingFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowBlendingFlatQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { BlendingMutateDialog } from './blending/blending-mutate-dialog'

function getBlendingColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<BlendingFlatRow> }>,
): ColumnDef<BlendingFlatRow>[] {
  return [

    { ...textColumn<BlendingFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<BlendingFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<BlendingFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...textColumn<BlendingFlatRow>('targetProductIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'doc' as const } },

    { ...textColumn<BlendingFlatRow>('itemType', t('common:columns.type'), { primary: false }), meta: { label: t('common:columns.type'), sizingCategory: 'capped', groupRole: 'item' as const } },
    { ...textColumn<BlendingFlatRow>('productIdName', t('common:table.product'), { primary: false }), id: 'itemProduct', meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<BlendingFlatRow>('storageIdName', t('common:columns.storage'), { primary: false }), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<BlendingFlatRow>('amount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<BlendingFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },

    { ...actionsColumn<BlendingFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const blendingRoute = getRouteApi('/_authenticated/internal/blending/')
const blendingDetailRoute = getRouteApi('/_authenticated/internal/blending/$id')
const blendingGlobalFilterFn = createGlobalFilter<BlendingFlatRow>('documentNumber', 'productIdName')

function BlendingTable({
  data,
  actions,
  RowActions,
}: {
  data: BlendingFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<BlendingFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="blending"
      data={data}
      getColumns={t => getBlendingColumns(t, RowActions)}
      routeApi={blendingRoute}
      globalFilterFn={blendingGlobalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
      actions={actions}
    />
  )
}

function useBlendingTitle() {
  return useTranslation(['common', 'documents']).t('common:nav.blending')
}

function useBlendingText() {
  const title = useBlendingTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('documents:document.blending'),
  }
}

interface BlendingDetailData {
  document: {
    id: string
    documentNumber: string
    status: string
    date: string
    contractorId?: string | null
    contractorIdName?: string | null
    targetProductId?: string | null
    targetProductIdName?: string | null
    executedAt?: string | null
  }
  components: BlendingComponentResponse[]
  results: BlendingResultResponse[]
}

const blendingViewDefinition = defineDocumentViews<BlendingFlatRow, BlendingDetailData>({
  displayName: 'Blending',
  useText: useBlendingText,
  useQuery: useFlowBlendingFlatQuery,
  Table: BlendingTable,
  MutateDialog: BlendingMutateDialog,
  deleteDialog: {
    hardDeleteFn: blendingDocumentHardDelete,
    softDeleteFn: blendingDocumentSoftDelete,
    queryKey: flowBlendingFlatQueryQueryKey,
    entityLabel: 'common:nav.blending',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: blendingDocumentExecute,
    revertFn: blendingDocumentRevert,
    queryKey: flowBlendingFlatQueryQueryKey,
  },
  rowActions: {
    getDetailPath: row => `/internal/blending/${row.documentId}`,
  },
  detail: {
    useDetailId: () => blendingDetailRoute.useParams().id,
    useDetailQuery: id => useBlendingCompositeGet(id, { embed: 'names' }),
    backTo: '/internal/blending',
    statusColorMap: statusColors,
    getDocument: data => ({
      id: data.document.id,
      documentNumber: data.document.documentNumber,
      status: data.document.status,
    }),
    renderFormContent: ({ data, t }) => {
      return (
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(data.document.date)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{data.document.contractorIdName ?? data.document.contractorId}</DetailField>
          <DetailField label={t('common:table.product')}>{data.document.targetProductIdName ?? data.document.targetProductId}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <>
          <ChildItemsTable
            items={data.components}
            columns={[
              textColumn<BlendingComponentResponse>('sourceProductIdName', t('common:table.product')),
              textColumn<BlendingComponentResponse>('storageIdName', t('common:columns.storage')),
              numericColumn<BlendingComponentResponse>('amountUsed', t('common:table.quantity')),
            ]}
            isLocked={isLocked}
            sectionTitle={t('blending:section.components')}
          />
          <ChildItemsTable
            items={data.results}
            columns={[
              textColumn<BlendingResultResponse>('storageIdName', t('common:columns.storage')),
              numericColumn<BlendingResultResponse>('producedAmount', t('common:table.quantity')),
            ]}
            isLocked={isLocked}
            sectionTitle={t('blending:section.results')}
          />
        </>
      )
    },
    renderMetadataContent: ({ data, t }) => {
      if (!data.document.executedAt) {
        return null
      }
      return (
        <div className="grid grid-cols-3 gap-4 text-sm">
          <div>
            <span className="text-muted-foreground">
              {t('documents:metadata.executedAt')}
              :
            </span>
            {' '}
            {formatDateTime(data.document.executedAt)}
          </div>
        </div>
      )
    },
  },
})

export function BlendingPage() {
  return <blendingViewDefinition.View />
}

export function BlendingDetail() {
  return <blendingViewDefinition.DetailView />
}
