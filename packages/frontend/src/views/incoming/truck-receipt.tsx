import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ReactNode } from 'react'
import type { ActionDescriptor } from '~/components/document/document-header'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceFlatRow, AcceptanceItemResponse, PipelineStatus, TruckReceiptPipelineResponse, TruckWaybillItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useState } from 'react'
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
import { canEditAcceptance, canEditBasis, canIssueAcceptance } from '~/lib/pipeline-policy'
import { AcceptanceMutateDialog } from './acceptance/acceptance-mutate-dialog'
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
    // 3 inline slots: View details (always), Edit (PENDING only), Issue
    // acceptance (PENDING only). With fewer slots the kebab takes the only
    // visible cell and the inline icons clip out of view.
    actionsColumn<TruckReceiptPipelineResponse>(RowActions, 3),
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

// `IssueAcceptanceDialog` is a secondary lifecycle slot — separate from the
// classic `LifecycleDialog` (execute/revert) — used here because the basis →
// action "Issue acceptance" trigger spawns a *different* document seeded with
// the row as its basis. The slot wires the row id into the
// AcceptanceMutateDialog's `prefillBasis` prop and fires when
// `openIssueAcceptance(row)` is dispatched. See design spec §3.2 / §6.3.
//
// Pipeline-row Edit visibility uses `row.pipelineStatus === 'PENDING'`, not
// `canEditBasis`. The row only exposes the basis edit affordance for PENDING;
// once an acceptance exists (DRAFT), editing the basis is reachable from the
// basis detail page (which uses the broader `canEditBasis` predicate).
const truckReceiptViewDefinition = defineCrudViews<TruckReceiptPipelineResponse>({
  displayName: 'TruckReceipt',
  useTitle: useTruckReceiptTitle,
  useQuery: useFlowTruckReceiptQuery,
  Table: TruckReceiptTableWithActions,
  MutateDialog: TruckReceiptMutateDialog,
  IssueAcceptanceDialog: AcceptanceMutateDialog,
  prefillBasisKind: 'truck',
  supportsUpdate: true,
  rowActions: {
    getDetailPath: row => `/incoming/truck/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
    pipelineActions: {
      editVisible: row => row.pipelineStatus === 'PENDING',
      issueAcceptance: { visible: row => canIssueAcceptance(row.pipelineStatus) },
    },
  },
})

export function TruckReceiptPage() {
  return <truckReceiptViewDefinition.View />
}

/**
 * Acceptance status keys are normalized via `lowerCaseStatus` to look up the
 * matching `documents:related.acceptance.{pending|draft|executed}` label.
 */
function lowerCaseStatus(s: string) {
  return s.toLowerCase() as 'pending' | 'draft' | 'executed'
}

/**
 * Adapt a fetched acceptance composite into the minimal `AcceptanceFlatRow`
 * shape the `AcceptanceMutateDialog` expects in edit mode (it only reads
 * `documentId` for the GET, but the prop is typed end-to-end).
 */
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

function useTruckReceiptDetailVariants(id: string) {
  const { t } = useTranslation(['common', 'documents', 'acceptance', 'truck-receipt'])
  const waybillQuery = useTransportTruckWaybillCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const acceptanceQuery = useAcceptanceCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const pipelineQuery = useFlowTruckReceiptQuery()
  const rows = pipelineQuery.data?.data ?? []

  const [editWaybillOpen, setEditWaybillOpen] = useState(false)
  const [issueAcceptanceOpen, setIssueAcceptanceOpen] = useState(false)
  const [editAcceptanceOpen, setEditAcceptanceOpen] = useState(false)

  // Acceptance variant
  let acceptanceContent: ReactNode
  if (acceptanceQuery.data?.data) {
    const doc = acceptanceQuery.data.data
    const matchingRow = rows.find(r => r.actionId === doc.id)
    const basisPipelineStatus: PipelineStatus = matchingRow?.pipelineStatus ?? 'DRAFT'
    const relatedDocs: RelatedDocument[] = []
    if (doc.truckWaybillId) {
      // The label derives from the basis pipeline state (spec §3.3): PENDING →
      // "pending acceptance"; DRAFT → "in draft"; EXECUTED → "accepted".
      relatedDocs.push({
        type: 'basis',
        label: t('documents:document.truckWaybill'),
        documentNumber: doc.truckWaybillIdName ?? doc.truckWaybillId,
        status: t(`documents:related.acceptance.${lowerCaseStatus(basisPipelineStatus)}`),
        statusColorMap: statusColors,
        to: `/incoming/truck/${doc.truckWaybillId}`,
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
            backTo: '/incoming/truck',
            executeFn: acceptanceDocumentExecute,
            revertFn: acceptanceDocumentRevert,
            queryKey: flowTruckReceiptQueryQueryKey(),
            statusColorMap: statusColors,
            actions: acceptanceActions,
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
        <AcceptanceMutateDialog
          open={editAcceptanceOpen}
          onOpenChange={open => setEditAcceptanceOpen(open)}
          currentRow={editAcceptanceOpen ? makeAcceptanceFlatRow(doc) : null}
        />
      </>
    )
  }

  // Basis (waybill) variant
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
        to: `/incoming/truck/${matchingRow.actionId}`,
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
            title: t('documents:document.truckWaybill'),
            entityLabel: t('documents:document.truckWaybill'),
            backTo: '/incoming/truck',
            statusColorMap: statusColors,
            actions: basisActions,
          }}
          // Status pill reflects the *pipeline* state, not a hardcoded
          // 'PENDING'. The basis itself has no own status column; pipeline
          // state is the single observable source.
          document={{ id: wb.id, documentNumber: wb.documentNumber, status: pipelineStatus }}
          subtitle={t('common:nav.truckReceipt')}
          relatedContent={basisRelated.length > 0 ? <RelatedDocuments documents={basisRelated} /> : undefined}
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
                  isLocked={!canEditBasis(pipelineStatus)}
                  sectionTitle={t('truck-receipt:section.items')}
                />
              )
            : undefined}
        />
        <TruckReceiptMutateDialog
          open={editWaybillOpen}
          onOpenChange={open => setEditWaybillOpen(open)}
          currentRow={editWaybillOpen && matchingRow ? matchingRow : null}
        />
        <AcceptanceMutateDialog
          open={issueAcceptanceOpen}
          onOpenChange={open => setIssueAcceptanceOpen(open)}
          prefillBasis={issueAcceptanceOpen ? { kind: 'truck', basisId: wb.id } : undefined}
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

const TruckReceiptResolvedDetail = defineResolvedDetailView({
  useDetailId: () => detailRoute.useParams().id,
  useVariants: useTruckReceiptDetailVariants,
  getNotFoundMessage: t => t('documents:messages.notFound'),
})

export function TruckReceiptDetail() {
  return <TruckReceiptResolvedDetail />
}
