import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceItemResponse, TruckReceiptPipelineResponse, TruckWaybillItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { RelatedDocuments } from '~/components/document/related-documents'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { acceptanceDocumentExecute, acceptanceDocumentRevert, transportTruckWaybillCreate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { flowTruckReceiptQueryQueryKey, useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<TruckReceiptPipelineResponse, DialogType>('TruckReceipt')

const DataTableRowActions = createRowActions<TruckReceiptPipelineResponse>({
  useEntity,
  disableEdit: true,
  getDetailPath: (row) => `/incoming/truck/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
})

function getColumns(t: TFunction): ColumnDef<TruckReceiptPipelineResponse>[] {
  return [
    textColumn<TruckReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber'), { sizing: 'capped', maxSize: 200 }),
    dateColumn<TruckReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<TruckReceiptPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }),
    textColumn<TruckReceiptPipelineResponse>('productName', t('common:table.product'), { primary: false }),
    numericColumn<TruckReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    textColumn<TruckReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber'), { primary: false, sizing: 'capped', maxSize: 200 }),
    numericColumn<TruckReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
    statusColumn<TruckReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), statusColors),
    actionsColumn<TruckReceiptPipelineResponse>(DataTableRowActions, 1),
  ]
}

const route = getRouteApi('/_authenticated/incoming/truck/')
const detailRoute = getRouteApi('/_authenticated/incoming/truck/$id')
const globalFilterFn = createGlobalFilter<TruckReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function TruckReceiptTable({ data, actions }: { data: TruckReceiptPipelineResponse[], actions?: React.ReactNode }) {
  return (
    <EntityTable
      tableId="truck-receipt"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      actions={actions}
    />
  )
}

// Waybill creation dialog
const waybillSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  senderId: z.string().uuid(),
})

type WaybillFormValues = z.infer<typeof waybillSchema>

function WaybillMutateDialog({ open, onOpenChange }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: TruckReceiptPipelineResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    schema: waybillSchema,
    defaultValues: { documentNumber: '', date: '', senderId: '' },
    createFn: transportTruckWaybillCreate,
    queryKey: flowTruckReceiptQueryQueryKey(),
    entityLabel: t('common:nav.truckReceipt'),
    formId: 'truck-waybill-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description={t('common:document.truckWaybill')} formId="truck-waybill-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="truck-waybill-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<WaybillFormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<WaybillFormValues> name="date" label={t('common:table.date')} type="date" />
          <EntityPickerField<WaybillFormValues> name="senderId" label={t('common:table.contractor')} queryResult={companiesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog: WaybillMutateDialog })

const PrimaryButtons = createPrimaryButtons({ useEntity })

export function TruckReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowTruckReceiptQuery()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.truckReceipt')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={TruckReceiptTable}
      dialogs={Dialogs}
    />
  )
}

export function TruckReceiptDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])

  // Try both: waybill (pending) and acceptance (draft/executed).
  // One will 404 depending on which type the ID belongs to — suppress error toasts for expected 404s.
  const waybillQuery = useTransportTruckWaybillCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const acceptanceQuery = useAcceptanceCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })

  const isLoading = waybillQuery.isLoading && acceptanceQuery.isLoading

  if (isLoading)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  // If acceptance found, show acceptance detail with related documents
  if (acceptanceQuery.data?.data) {
    const doc = acceptanceQuery.data.data
    return (
      <DocumentDetailPage
        config={{
          title: t('common:document.acceptance'),
          entityLabel: t('common:document.acceptance'),
          backTo: '/incoming/truck',
          executeFn: acceptanceDocumentExecute,
          revertFn: acceptanceDocumentRevert,
          queryKey: flowTruckReceiptQueryQueryKey(),
          statusColorMap: statusColors,
        }}
        document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
        subtitle={t('common:nav.truckReceipt')}
        relatedContent={(() => {
          const docs: RelatedDocument[] = []
          if (doc.truckWaybillId) {
            docs.push({ type: 'basis', label: t('common:document.truckWaybill'), documentNumber: doc.truckWaybillIdName ?? doc.truckWaybillId, status: t('common:document.pendingAcceptance'), statusColorMap: statusColors, to: `/incoming/truck/${doc.truckWaybillId}` })
          }
          return <RelatedDocuments documents={docs} />
        })()}
        formContent={(
          <div className="grid grid-cols-3 gap-4">
            <div>
              <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
              <p>{formatDate(doc.dateAccepted)}</p>
            </div>
            <div>
              <span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span>
              <p>{doc.contractorIdName ?? '—'}</p>
            </div>
            <div>
              <span className="text-sm text-muted-foreground">{t('common:table.source')}</span>
              <p>{doc.sourceEntity ?? '—'}</p>
            </div>
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
            sectionTitle={t('common:sections.acceptanceItems')}
          />
        )}
        metadataContent={doc.executedAt
          ? (
              <div className="text-sm">
                <span className="text-muted-foreground">{t('common:metadata.executedAt')}:</span>
                {' '}
                {formatDateTime(doc.executedAt)}
              </div>
            )
          : null}
      />
    )
  }

  // Otherwise show waybill detail (pending)
  if (waybillQuery.data?.data) {
    const composite = waybillQuery.data.data
    const wb = composite.waybill
    const noOp = async () => {}
    return (
      <DocumentDetailPage
        config={{ title: t('common:document.truckWaybill'), entityLabel: t('common:document.truckWaybill'), backTo: '/incoming/truck', executeFn: noOp, revertFn: noOp, queryKey: flowTruckReceiptQueryQueryKey(), statusColorMap: statusColors }}
        document={{ id: wb.id, documentNumber: wb.documentNumber, status: 'PENDING' }}
        subtitle={t('common:nav.truckReceipt')}
        formContent={(
          <div className="grid grid-cols-3 gap-4">
            <div>
              <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
              <p>{formatDate(wb.date)}</p>
            </div>
            <div>
              <span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span>
              <p>{wb.senderIdName ?? wb.senderId}</p>
            </div>
          </div>
        )}
        itemsContent={composite.items?.length ? (
          <ChildItemsTable
            items={composite.items}
            columns={[
              textColumn<TruckWaybillItemResponse>('productIdName', t('common:table.product')),
              numericColumn<TruckWaybillItemResponse>('declaredAmount', t('common:table.declaredQty')),
            ]}
            isLocked={false}
            sectionTitle={t('common:sections.waybillItems')}
          />
        ) : undefined}
      />
    )
  }

  return <div className="p-4">Document not found</div>
}
