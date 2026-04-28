import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { InventoryAdjustmentResponse, ReconciliationFlatRow } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { reconciliationExecute, reconciliationHardDelete, reconciliationRevert, reconciliationSoftDelete } from '~/generated/client'
import { useAdjustmentList } from '~/generated/hooks/DocumentOperationsHooks/useAdjustmentList'
import { useReconciliationGet } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationGet'
import { flowReconciliationFlatQueryQueryKey, useFlowReconciliationFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowReconciliationFlatQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { ReconciliationMutateDialog } from './reconciliation/reconciliation-mutate-dialog'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<ReconciliationFlatRow> }>,
): ColumnDef<ReconciliationFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<ReconciliationFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<ReconciliationFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<ReconciliationFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...textColumn<ReconciliationFlatRow>('warehouseIdName', t('common:columns.warehouse'), { primary: false }), meta: { label: t('common:columns.warehouse'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<ReconciliationFlatRow>('productIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<ReconciliationFlatRow>('storageIdName', t('common:columns.storage'), { primary: false }), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<ReconciliationFlatRow>('adjustmentType', t('common:columns.type'), { primary: false }), meta: { label: t('common:columns.type'), sizingCategory: 'capped', groupRole: 'item' as const } },
    { ...numericColumn<ReconciliationFlatRow>('amount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...textColumn<ReconciliationFlatRow>('reason', t('common:columns.reason'), { primary: false }), meta: { label: t('common:columns.reason'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...statusColumn<ReconciliationFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Actions (doc-level)
    { ...actionsColumn<ReconciliationFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/internal/reconciliation/')
const detailRoute = getRouteApi('/_authenticated/internal/reconciliation/$id')
const globalFilterFn = createGlobalFilter<ReconciliationFlatRow>('documentNumber', 'productIdName')

function ReconciliationTable({
  data,
  actions,
  RowActions,
}: {
  data: ReconciliationFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<ReconciliationFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="reconciliation"
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

function useReconciliationTitle() {
  return useTranslation(['common', 'documents']).t('common:nav.reconciliation')
}

function useReconciliationText() {
  const title = useReconciliationTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('documents:document.reconciliation'),
  }
}

interface ReconciliationDocumentDetailData {
  doc: {
    id: string
    documentNumber: string
    status: string
    date: string
    contractorIdName?: string | null
    warehouseId: string
    warehouseIdName?: string | null
    executedAt?: string | null
  }
  items: InventoryAdjustmentResponse[]
}

const reconciliationViewDefinition = defineDocumentViews<ReconciliationFlatRow, ReconciliationDocumentDetailData>({
  displayName: 'Reconciliation',
  useText: useReconciliationText,
  useQuery: useFlowReconciliationFlatQuery,
  Table: ReconciliationTable,
  MutateDialog: ReconciliationMutateDialog,
  deleteDialog: {
    hardDeleteFn: reconciliationHardDelete,
    softDeleteFn: reconciliationSoftDelete,
    queryKey: flowReconciliationFlatQueryQueryKey,
    entityLabel: 'common:nav.reconciliation',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: reconciliationExecute,
    revertFn: reconciliationRevert,
    queryKey: flowReconciliationFlatQueryQueryKey,
  },
  rowActions: {
    getDetailPath: row => `/internal/reconciliation/${row.documentId}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: (id) => {
      const docResult = useReconciliationGet(id, { embed: 'names' })
      const itemsResult = useAdjustmentList()

      return {
        isLoading: docResult.isLoading,
        data: docResult.data?.data
          ? {
              data: {
                doc: docResult.data.data,
                items: (itemsResult.data?.data ?? []).filter((item: InventoryAdjustmentResponse) => item.reconciliationId === id),
              },
            }
          : undefined,
      }
    },
    backTo: '/internal/reconciliation',
    statusColorMap: statusColors,
    getDocument: data => ({
      id: data.doc.id,
      documentNumber: data.doc.documentNumber,
      status: data.doc.status,
    }),
    renderFormContent: ({ data, t }) => {
      return (
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(data.doc.date)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{data.doc.contractorIdName ?? '—'}</DetailField>
          <DetailField label={t('common:columns.warehouse')}>{data.doc.warehouseIdName ?? data.doc.warehouseId}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <ChildItemsTable
          items={data.items}
          columns={[
            textColumn<InventoryAdjustmentResponse>('productIdName', t('common:table.product')),
            textColumn<InventoryAdjustmentResponse>('storageIdName', t('common:columns.storage')),
            textColumn<InventoryAdjustmentResponse>('adjustmentType', t('common:columns.type')),
            numericColumn<InventoryAdjustmentResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={isLocked}
          sectionTitle={t('reconciliation:section.adjustments')}
        />
      )
    },
    renderMetadataContent: ({ data, t }) => {
      if (!data.doc.executedAt) {
        return null
      }
      return (
        <div className="text-sm">
          <span className="text-muted-foreground">
            {t('documents:metadata.executedAt')}
            :
          </span>
          {' '}
          {formatDateTime(data.doc.executedAt)}
        </div>
      )
    },
  },
})

export function ReconciliationPage() {
  return <reconciliationViewDefinition.View />
}

export function ReconciliationDetail() {
  return <reconciliationViewDefinition.DetailView />
}
