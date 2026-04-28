import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceItemResponse, TruckReceiptPipelineResponse, TruckWaybillItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField, DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { RelatedDocuments } from '~/components/document/related-documents'
import { acceptanceDocumentExecute, acceptanceDocumentRevert } from '~/generated/client'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { flowTruckReceiptQueryQueryKey, useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineCrudViews } from '~/lib/define-crud-views'
import { defineResolvedDetailView } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { TruckReceiptMutateDialog } from './truck-receipt/truck-receipt-mutate-dialog'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<TruckReceiptPipelineResponse> }>,
): ColumnDef<TruckReceiptPipelineResponse>[] {
  return [
    textColumn<TruckReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber'), { sizing: 'capped', maxSize: 200 }),
    dateColumn<TruckReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<TruckReceiptPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }),
    textColumn<TruckReceiptPipelineResponse>('productName', t('common:table.product'), { primary: false }),
    numericColumn<TruckReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    textColumn<TruckReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber'), { primary: false, sizing: 'capped', maxSize: 200 }),
    numericColumn<TruckReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
    statusColumn<TruckReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), statusColors),
    actionsColumn<TruckReceiptPipelineResponse>(RowActions, 1),
  ]
}

const route = getRouteApi('/_authenticated/incoming/truck/')
const detailRoute = getRouteApi('/_authenticated/incoming/truck/$id')
const globalFilterFn = createGlobalFilter<TruckReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function TruckReceiptTableWithActions({
  data,
  actions,
  RowActions,
}: {
  data: TruckReceiptPipelineResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<TruckReceiptPipelineResponse> }>
}) {
  return (
    <EntityTable
      tableId="truck-receipt"
      data={data}
      getColumns={t => getColumns(t, RowActions)}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      actions={actions}
    />
  )
}

function useTruckReceiptTitle() {
  return useTranslation(['common', 'documents']).t('common:nav.truckReceipt')
}

const truckReceiptViewDefinition = defineCrudViews<TruckReceiptPipelineResponse>({
  displayName: 'TruckReceipt',
  useTitle: useTruckReceiptTitle,
  useQuery: useFlowTruckReceiptQuery,
  Table: TruckReceiptTableWithActions,
  MutateDialog: TruckReceiptMutateDialog,
  supportsUpdate: false,
  rowActions: {
    disableEdit: true,
    getDetailPath: row => `/incoming/truck/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
  },
})

export function TruckReceiptPage() {
  return <truckReceiptViewDefinition.View />
}

function useTruckReceiptDetailVariants(id: string) {
  const { t } = useTranslation(['common'])
  const waybillQuery = useTransportTruckWaybillCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const acceptanceQuery = useAcceptanceCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })

  return [
    {
      isLoading: acceptanceQuery.isLoading,
      content: acceptanceQuery.data?.data
        ? (() => {
            const doc = acceptanceQuery.data.data
            const relatedDocs: RelatedDocument[] = []
            if (doc.truckWaybillId) {
              relatedDocs.push({ type: 'basis', label: t('documents:document.truckWaybill'), documentNumber: doc.truckWaybillIdName ?? doc.truckWaybillId, status: t('documents:document.pendingAcceptance'), statusColorMap: statusColors, to: `/incoming/truck/${doc.truckWaybillId}` })
            }
            return (
              <DocumentDetailPage
                config={{
                  title: t('documents:document.acceptance'),
                  entityLabel: t('documents:document.acceptance'),
                  backTo: '/incoming/truck',
                  executeFn: acceptanceDocumentExecute,
                  revertFn: acceptanceDocumentRevert,
                  queryKey: flowTruckReceiptQueryQueryKey(),
                  statusColorMap: statusColors,
                }}
                document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
                subtitle={t('common:nav.truckReceipt')}
                relatedContent={<RelatedDocuments documents={relatedDocs} />}
                formContent={(
                  <div className="grid grid-cols-3 gap-4">
                    <DetailField label={t('common:table.date')}>{formatDate(doc.dateAccepted)}</DetailField>
                    <DetailField label={t('common:table.contractor')}>{doc.contractorIdName ?? '—'}</DetailField>
                    <DetailField label={t('common:table.source')}>{doc.sourceEntity ?? '—'}</DetailField>
                  </div>
                )}
                itemsContent={(
                  <ChildItemsTable
                    items={doc.items}
                    columns={[
                      textColumn<AcceptanceItemResponse>('productIdName', t('common:table.product')),
                      textColumn<AcceptanceItemResponse>('storageIdName', t('common:columns.storage')),
                      numericColumn<AcceptanceItemResponse>('acceptedAmount', t('common:table.quantity')),
                    ]}
                    isLocked={doc.status === 'EXECUTED'}
                    sectionTitle={t('acceptance:section.items')}
                  />
                )}
                metadataContent={doc.executedAt
                  ? (
                      <div className="text-sm">
                        <span className="text-muted-foreground">
                          {t('documents:metadata.executedAt')}
                          :
                        </span>
                        {' '}
                        {formatDateTime(doc.executedAt)}
                      </div>
                    )
                  : null}
              />
            )
          })()
        : undefined,
    },
    {
      isLoading: waybillQuery.isLoading,
      content: waybillQuery.data?.data
        ? (() => {
            const composite = waybillQuery.data.data
            const wb = composite.waybill
            return (
              <DocumentDetailPage
                config={{
                  title: t('documents:document.truckWaybill'),
                  entityLabel: t('documents:document.truckWaybill'),
                  backTo: '/incoming/truck',
                  statusColorMap: statusColors,
                }}
                document={{ id: wb.id, documentNumber: wb.documentNumber, status: 'PENDING' }}
                subtitle={t('common:nav.truckReceipt')}
                formContent={(
                  <div className="grid grid-cols-3 gap-4">
                    <DetailField label={t('common:table.date')}>{formatDate(wb.date)}</DetailField>
                    <DetailField label={t('common:table.contractor')}>{wb.senderIdName ?? wb.senderId}</DetailField>
                  </div>
                )}
                itemsContent={composite.items?.length
                  ? (
                      <ChildItemsTable
                        items={composite.items}
                        columns={[
                          textColumn<TruckWaybillItemResponse>('productIdName', t('common:table.product')),
                          numericColumn<TruckWaybillItemResponse>('declaredAmount', t('common:table.declaredQty')),
                        ]}
                        isLocked={false}
                        sectionTitle={t('truck-receipt:section.items')}
                      />
                    )
                  : undefined}
              />
            )
          })()
        : undefined,
    },
  ]
}

const TruckReceiptResolvedDetail = defineResolvedDetailView({
  useDetailId: () => detailRoute.useParams().id,
  useVariants: useTruckReceiptDetailVariants,
  getNotFoundMessage: t => t('documents:messages.notFound'),
})

export function TruckReceiptDetail() {
  return <TruckReceiptResolvedDetail />
}
