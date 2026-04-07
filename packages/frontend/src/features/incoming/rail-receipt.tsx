import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceItemResponse, RailReceiptPipelineResponse, RailWagonManifestResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField, DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { RelatedDocuments } from '~/components/document/related-documents'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { acceptanceDocumentExecute, acceptanceDocumentRevert, transportRailWaybillCreate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { railReceiptPipelineQueryQueryKey, useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'
import { formatDate } from '~/lib/formatters'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<RailReceiptPipelineResponse, DialogType>('RailReceipt')

const DataTableRowActions = createRowActions<RailReceiptPipelineResponse>({
  useEntity,
  disableEdit: true,
  getDetailPath: row => `/incoming/rail/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
})

function getColumns(t: TFunction): ColumnDef<RailReceiptPipelineResponse>[] {
  return [
    textColumn<RailReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber'), { sizing: 'capped', maxSize: 200 }),
    dateColumn<RailReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<RailReceiptPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }),
    textColumn<RailReceiptPipelineResponse>('productName', t('common:table.product'), { primary: false }),
    numericColumn<RailReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    textColumn<RailReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber'), { primary: false, sizing: 'capped', maxSize: 200 }),
    numericColumn<RailReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
    statusColumn<RailReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), statusColors),
    actionsColumn<RailReceiptPipelineResponse>(DataTableRowActions, 1),
  ]
}

const routeApi = getRouteApi('/_authenticated/incoming/rail/')
const detailRoute = getRouteApi('/_authenticated/incoming/rail/$id')
const globalFilterFn = createGlobalFilter<RailReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function RailReceiptTable({ data, actions }: { data: RailReceiptPipelineResponse[], actions?: React.ReactNode }) {
  return <EntityTable tableId="rail-receipt" data={data} getColumns={getColumns} routeApi={routeApi} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} actions={actions} />
}

const waybillSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  senderId: z.string().uuid(),
})

type WaybillFormValues = z.infer<typeof waybillSchema>

function WaybillMutateDialog({ open, onOpenChange }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: RailReceiptPipelineResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    schema: waybillSchema,
    defaultValues: { documentNumber: '', date: '', senderId: '' },
    createFn: transportRailWaybillCreate,
    queryKey: railReceiptPipelineQueryQueryKey(),
    entityLabel: t('common:nav.railReceipt'),
    formId: 'rail-waybill-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description={t('common:document.railWaybill')} formId="rail-waybill-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="rail-waybill-form" onSubmit={handleSubmit} className="space-y-5">
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

export function RailReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useRailReceiptPipelineQuery()

  return <EntityPage provider={Provider} title={t('common:nav.railReceipt')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={RailReceiptTable} dialogs={Dialogs} />
}

export function RailReceiptDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const waybillQuery = useTransportRailWaybillCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const acceptanceQuery = useAcceptanceCompositeGet(id, { embed: 'names' }, { query: { retry: false, meta: { suppressErrorToast: true } } })
  const isLoading = waybillQuery.isLoading && acceptanceQuery.isLoading

  if (isLoading)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  if (acceptanceQuery.data?.data) {
    const doc = acceptanceQuery.data.data
    const relatedDocs: RelatedDocument[] = []
    if (doc.railWaybillId) {
      relatedDocs.push({ type: 'basis', label: t('common:document.railWaybill'), documentNumber: doc.railWaybillIdName ?? doc.railWaybillId, status: t('common:document.pendingAcceptance'), statusColorMap: statusColors, to: `/incoming/rail/${doc.railWaybillId}` })
    }
    return (
      <DocumentDetailPage
        config={{ title: t('common:document.acceptance'), entityLabel: t('common:document.acceptance'), backTo: '/incoming/rail', executeFn: acceptanceDocumentExecute, revertFn: acceptanceDocumentRevert, queryKey: railReceiptPipelineQueryQueryKey(), statusColorMap: statusColors }}
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
            sectionTitle={t('common:sections.acceptanceItems')}
          />
        )}
        metadataContent={doc.executedAt
          ? (
              <div className="text-sm">
                <span className="text-muted-foreground">
                  {t('common:metadata.executedAt')}
                  :
                </span>
                {' '}
                {doc.executedAt}
              </div>
            )
          : null}
      />
    )
  }

  if (waybillQuery.data?.data) {
    const composite = waybillQuery.data.data
    const wb = composite.waybill
    const noOp = async () => {}
    return (
      <DocumentDetailPage
        config={{ title: t('common:document.railWaybill'), entityLabel: t('common:document.railWaybill'), backTo: '/incoming/rail', executeFn: noOp, revertFn: noOp, queryKey: railReceiptPipelineQueryQueryKey(), statusColorMap: statusColors }}
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
                sectionTitle={t('common:sections.wagonManifests')}
              />
            )
          : undefined}
      />
    )
  }

  return <div className="p-4">Document not found</div>
}
