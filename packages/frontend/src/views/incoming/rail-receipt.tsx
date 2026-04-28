import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceItemResponse, RailReceiptPipelineResponse, RailWagonManifestResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField, DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { RelatedDocuments } from '~/components/document/related-documents'
import { acceptanceDocumentExecute, acceptanceDocumentRevert } from '~/generated/client'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { railReceiptPipelineQueryQueryKey, useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineCrudViews } from '~/lib/define-crud-views'
import { defineResolvedDetailView } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { RailReceiptMutateDialog } from './rail-receipt/rail-receipt-mutate-dialog'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<RailReceiptPipelineResponse> }>,
): ColumnDef<RailReceiptPipelineResponse>[] {
  return [
    textColumn<RailReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber'), { sizing: 'capped', maxSize: 200 }),
    dateColumn<RailReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<RailReceiptPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }),
    textColumn<RailReceiptPipelineResponse>('productName', t('common:table.product'), { primary: false }),
    numericColumn<RailReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    textColumn<RailReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber'), { primary: false, sizing: 'capped', maxSize: 200 }),
    numericColumn<RailReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
    statusColumn<RailReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), statusColors),
    actionsColumn<RailReceiptPipelineResponse>(RowActions, 1),
  ]
}

const routeApi = getRouteApi('/_authenticated/incoming/rail/')
const detailRoute = getRouteApi('/_authenticated/incoming/rail/$id')
const globalFilterFn = createGlobalFilter<RailReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function RailReceiptTable({
  data,
  actions,
  RowActions,
}: {
  data: RailReceiptPipelineResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<RailReceiptPipelineResponse> }>
}) {
  return <EntityTable tableId="rail-receipt" data={data} getColumns={t => getColumns(t, RowActions)} routeApi={routeApi} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} actions={actions} />
}

function useRailReceiptTitle() {
  return useTranslation(['common', 'documents']).t('common:nav.railReceipt')
}

const railReceiptViewDefinition = defineCrudViews<RailReceiptPipelineResponse>({
  displayName: 'RailReceipt',
  useTitle: useRailReceiptTitle,
  useQuery: useRailReceiptPipelineQuery,
  Table: RailReceiptTable,
  MutateDialog: RailReceiptMutateDialog,
  supportsUpdate: false,
  rowActions: {
    disableEdit: true,
    getDetailPath: row => `/incoming/rail/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
  },
})

export function RailReceiptPage() {
  return <railReceiptViewDefinition.View />
}

function useRailReceiptDetailVariants(id: string) {
  const { t } = useTranslation(['common'])
  const waybillQuery = useTransportRailWaybillCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const acceptanceQuery = useAcceptanceCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })

  return [
    {
      isLoading: acceptanceQuery.isLoading,
      content: acceptanceQuery.data?.data
        ? (() => {
            const doc = acceptanceQuery.data.data
            const relatedDocs: RelatedDocument[] = []
            if (doc.railWaybillId) {
              relatedDocs.push({ type: 'basis', label: t('documents:document.railWaybill'), documentNumber: doc.railWaybillIdName ?? doc.railWaybillId, status: t('documents:document.pendingAcceptance'), statusColorMap: statusColors, to: `/incoming/rail/${doc.railWaybillId}` })
            }
            return (
              <DocumentDetailPage
                config={{ title: t('documents:document.acceptance'), entityLabel: t('documents:document.acceptance'), backTo: '/incoming/rail', executeFn: acceptanceDocumentExecute, revertFn: acceptanceDocumentRevert, queryKey: railReceiptPipelineQueryQueryKey(), statusColorMap: statusColors }}
                document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
                subtitle={t('common:nav.railReceipt')}
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
                config={{ title: t('documents:document.railWaybill'), entityLabel: t('documents:document.railWaybill'), backTo: '/incoming/rail', statusColorMap: statusColors }}
                document={{ id: wb.id, documentNumber: wb.documentNumber, status: 'PENDING' }}
                subtitle={t('common:nav.railReceipt')}
                formContent={(
                  <div className="grid grid-cols-3 gap-4">
                    <DetailField label={t('common:table.date')}>{formatDate(wb.date)}</DetailField>
                    <DetailField label={t('common:table.contractor')}>{wb.senderIdName ?? wb.senderId}</DetailField>
                  </div>
                )}
                itemsContent={composite.wagonManifests?.length
                  ? (
                      <ChildItemsTable
                        items={composite.wagonManifests}
                        columns={[
                          textColumn<RailWagonManifestResponse>('productIdName', t('common:table.product')),
                          textColumn<RailWagonManifestResponse>('wagonNumber', t('common:columns.wagonNumber')),
                          numericColumn<RailWagonManifestResponse>('declaredMass', t('common:table.declaredQty')),
                        ]}
                        isLocked={false}
                        sectionTitle={t('rail-receipt:section.manifests')}
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

const RailReceiptResolvedDetail = defineResolvedDetailView({
  useDetailId: () => detailRoute.useParams().id,
  useVariants: useRailReceiptDetailVariants,
  getNotFoundMessage: t => t('documents:messages.notFound'),
})

export function RailReceiptDetail() {
  return <RailReceiptResolvedDetail />
}
