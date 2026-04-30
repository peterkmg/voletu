import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ReactNode } from 'react'
import type { ActionDescriptor } from '~/components/document/document-header'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceFlatRow, AcceptanceItemResponse, PipelineStatus, RailReceiptPipelineResponse, RailWagonManifestResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useState } from 'react'
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
import { canEditAcceptance, canEditBasis, canIssueAcceptance } from '~/lib/pipeline-policy'
import { AcceptanceMutateDialog } from './acceptance/acceptance-mutate-dialog'
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

    actionsColumn<RailReceiptPipelineResponse>(RowActions, 3),
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
  IssueAcceptanceDialog: AcceptanceMutateDialog,
  prefillBasisKind: 'rail',
  supportsUpdate: true,
  rowActions: {
    getDetailPath: row => `/incoming/rail/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
    pipelineActions: {
      editVisible: row => row.pipelineStatus === 'PENDING',
      issueAcceptance: { visible: row => canIssueAcceptance(row.pipelineStatus) },
    },
  },
})

export function RailReceiptPage() {
  return <railReceiptViewDefinition.View />
}

function lowerCaseStatus(s: string) {
  return s.toLowerCase() as 'pending' | 'draft' | 'executed'
}

function makeAcceptanceFlatRow(
  doc: { id: string, documentNumber: string, status: AcceptanceFlatRow['status'], dateAccepted: string, contractorIdName?: string | null, sourceEntity?: string | null },
): AcceptanceFlatRow {
  return {
    acceptedAmount: '',
    contractorIdName: doc.contractorIdName ?? '',
    dateAccepted: doc.dateAccepted,
    documentId: doc.id,
    documentNumber: doc.documentNumber,
    id: doc.id,
    itemId: doc.id,
    productIdName: '',
    sourceEntity: doc.sourceEntity ?? null,
    status: doc.status,
    storageIdName: '',
  }
}

function useRailReceiptDetailVariants(id: string) {
  const { t } = useTranslation(['common', 'documents', 'acceptance', 'rail-receipt'])
  const waybillQuery = useTransportRailWaybillCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const acceptanceQuery = useAcceptanceCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const pipelineQuery = useRailReceiptPipelineQuery()
  const rows = pipelineQuery.data?.data ?? []

  const [editWaybillOpen, setEditWaybillOpen] = useState(false)
  const [issueAcceptanceOpen, setIssueAcceptanceOpen] = useState(false)
  const [editAcceptanceOpen, setEditAcceptanceOpen] = useState(false)

  let acceptanceContent: ReactNode
  if (acceptanceQuery.data?.data) {
    const doc = acceptanceQuery.data.data
    const matchingRow = rows.find(r => r.actionId === doc.id)
    const basisPipelineStatus: PipelineStatus = matchingRow?.pipelineStatus ?? 'DRAFT'
    const relatedDocs: RelatedDocument[] = []
    if (doc.railWaybillId) {
      relatedDocs.push({
        type: 'basis',
        label: t('documents:document.railWaybill'),
        documentNumber: doc.railWaybillIdName ?? doc.railWaybillId,
        status: t(`documents:related.acceptance.${lowerCaseStatus(basisPipelineStatus)}`),
        statusColorMap: statusColors,
        to: `/incoming/rail/${doc.railWaybillId}`,
      })
    }
    const acceptanceActions: ActionDescriptor[] = [
      {
        id: 'edit-acceptance',
        label: t('documents:actions.edit'),
        onClick: () => setEditAcceptanceOpen(true),
        disabled: !canEditAcceptance(doc.status),
        disabledReason: canEditAcceptance(doc.status) ? undefined : t('documents:reasons.executedLocked'),
      },
    ]
    acceptanceContent = (
      <>
        <DocumentDetailPage
          config={{
            title: t('documents:document.acceptance'),
            entityLabel: t('documents:document.acceptance'),
            backTo: '/incoming/rail',
            executeFn: acceptanceDocumentExecute,
            revertFn: acceptanceDocumentRevert,
            queryKey: railReceiptPipelineQueryQueryKey(),
            statusColorMap: statusColors,
            actions: acceptanceActions,
          }}
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
        <AcceptanceMutateDialog
          open={editAcceptanceOpen}
          onOpenChange={open => setEditAcceptanceOpen(open)}
          currentRow={editAcceptanceOpen ? makeAcceptanceFlatRow(doc) : null}
        />
      </>
    )
  }

  let basisContent: ReactNode
  if (waybillQuery.data?.data) {
    const composite = waybillQuery.data.data
    const wb = composite.waybill
    const matchingRow = rows.find(r => r.id === wb.id)
    const pipelineStatus: PipelineStatus = matchingRow?.pipelineStatus ?? 'PENDING'
    const basisRelated: RelatedDocument[] = []
    if (matchingRow?.actionId) {
      basisRelated.push({
        type: 'reference',
        label: t('documents:document.acceptance'),
        documentNumber: matchingRow.actionDocumentNumber ?? matchingRow.actionId,
        status: t(`documents:related.acceptance.${lowerCaseStatus(pipelineStatus)}`),
        statusColorMap: statusColors,
        to: `/incoming/rail/${matchingRow.actionId}`,
      })
    }
    const basisActions: ActionDescriptor[] = [
      {
        id: 'edit-basis',
        label: t('documents:actions.edit'),
        onClick: () => setEditWaybillOpen(true),
        disabled: !canEditBasis(pipelineStatus),
        disabledReason: canEditBasis(pipelineStatus) ? undefined : t('documents:reasons.executedLocked'),
      },
      {
        id: 'issue-acceptance',
        label: t('documents:actions.issueAcceptance'),
        onClick: () => setIssueAcceptanceOpen(true),
        disabled: !canIssueAcceptance(pipelineStatus),
        disabledReason: canIssueAcceptance(pipelineStatus)
          ? undefined
          : t('documents:reasons.alreadyIssued', { ref: matchingRow?.actionDocumentNumber ?? '' }),
        variant: 'primary',
      },
    ]
    basisContent = (
      <>
        <DocumentDetailPage
          config={{
            title: t('documents:document.railWaybill'),
            entityLabel: t('documents:document.railWaybill'),
            backTo: '/incoming/rail',
            statusColorMap: statusColors,
            actions: basisActions,
          }}
          document={{ id: wb.id, documentNumber: wb.documentNumber, status: pipelineStatus }}
          subtitle={t('common:nav.railReceipt')}
          relatedContent={basisRelated.length > 0 ? <RelatedDocuments documents={basisRelated} /> : undefined}
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
                  isLocked={!canEditBasis(pipelineStatus)}
                  sectionTitle={t('rail-receipt:section.manifests')}
                />
              )
            : undefined}
        />
        <RailReceiptMutateDialog
          open={editWaybillOpen}
          onOpenChange={open => setEditWaybillOpen(open)}
          currentRow={editWaybillOpen && matchingRow ? matchingRow : null}
        />
        <AcceptanceMutateDialog
          open={issueAcceptanceOpen}
          onOpenChange={open => setIssueAcceptanceOpen(open)}
          prefillBasis={issueAcceptanceOpen ? { kind: 'rail', basisId: wb.id } : undefined}
        />
      </>
    )
  }

  return [
    {
      isLoading: acceptanceQuery.isLoading,
      content: acceptanceContent,
    },
    {
      isLoading: waybillQuery.isLoading,
      content: basisContent,
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
