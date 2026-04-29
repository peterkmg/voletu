import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckDispatchMutationTarget } from './truck-dispatch/truck-dispatch-mutate-dialog'
import type { DispatchItemResponse, TruckDispatchPipelineResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { dispatchDocumentExecute, dispatchDocumentRevert } from '~/generated/client'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { truckDispatchPipelineQueryQueryKey, useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { TruckDispatchMutateDialog } from './truck-dispatch/truck-dispatch-mutate-dialog'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<TruckDispatchPipelineResponse> }>,
): ColumnDef<TruckDispatchPipelineResponse>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<TruckDispatchPipelineResponse>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<TruckDispatchPipelineResponse>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<TruckDispatchPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<TruckDispatchPipelineResponse>('productName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<TruckDispatchPipelineResponse>('dispatchedQuantity', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<TruckDispatchPipelineResponse>('pipelineStatus', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Actions (doc-level)
    { ...actionsColumn<TruckDispatchPipelineResponse>(RowActions, 1), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const routeApi = getRouteApi('/_authenticated/outgoing/truck/')
const detailRoute = getRouteApi('/_authenticated/outgoing/truck/$id')
const globalFilterFn = createGlobalFilter<TruckDispatchPipelineResponse>('documentNumber', 'contractorName')

function TruckDispatchTable({
  data,
  actions,
  RowActions,
}: {
  data: TruckDispatchPipelineResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<TruckDispatchPipelineResponse> }>
}) {
  return (
    <EntityTable
      tableId="truck-dispatch"
      data={data}
      getColumns={t => getColumns(t, RowActions)}
      routeApi={routeApi}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="id"
      actions={actions}
    />
  )
}

function useTruckDispatchTitle() {
  return useTranslation(['common', 'documents']).t('common:nav.truckDispatch')
}

function useTruckDispatchText() {
  const title = useTruckDispatchTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('documents:document.truckDispatch'),
  }
}

interface TruckDispatchDetailData {
  id: string
  documentNumber: string
  status: string
  date: string
  contractorId?: string | null
  contractorIdName?: string | null
  items: DispatchItemResponse[]
  executedAt?: string | null
}

/**
 * Adapter that lets the shared `defineDocumentViews` invoke the composite
 * truck-dispatch dialog. The pipeline view ships rows of
 * `TruckDispatchPipelineResponse`, but the dialog is keyed on the document
 * id (`row.id` here) and does not need the full pipeline payload, so we
 * project to the minimal mutation target shape the dialog expects.
 */
function TruckDispatchMutateDialogAdapter({
  open,
  onOpenChange,
  currentRow,
}: {
  open: boolean
  onOpenChange: (o: boolean) => void
  currentRow?: TruckDispatchPipelineResponse | null
}) {
  const adaptedRow: TruckDispatchMutationTarget | null = currentRow
    ? { documentId: currentRow.id }
    : null
  return (
    <TruckDispatchMutateDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={adaptedRow}
    />
  )
}

const truckDispatchViewDefinition = defineDocumentViews<TruckDispatchPipelineResponse, TruckDispatchDetailData>({
  displayName: 'TruckDispatch',
  useText: useTruckDispatchText,
  useQuery: useTruckDispatchPipelineQuery,
  Table: TruckDispatchTable,
  MutateDialog: TruckDispatchMutateDialogAdapter,
  supportsUpdate: false,
  documentActions: {
    executeFn: dispatchDocumentExecute,
    revertFn: dispatchDocumentRevert,
    queryKey: truckDispatchPipelineQueryQueryKey,
  },
  rowActions: {
    disableEdit: true,
    getDetailPath: row => `/outgoing/truck/${row.id}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: id => useDispatchCompositeGet(id, { embed: 'names' }),
    backTo: '/outgoing/truck',
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
          sectionTitle={t('truck-dispatch:section.items')}
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

export function TruckDispatchPage() {
  return <truckDispatchViewDefinition.View />
}

export function TruckDispatchDetail() {
  return <truckDispatchViewDefinition.DetailView />
}
