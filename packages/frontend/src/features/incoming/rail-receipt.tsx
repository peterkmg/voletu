import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RelatedDocument } from '~/components/document/related-documents'
import type { AcceptanceItemResponse, RailReceiptPipelineResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { DocumentDetailPage } from '~/components/document'
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
import { useTransportRailWaybillGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillGet'
import { railReceiptPipelineQueryQueryKey, useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors, pipelineStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<RailReceiptPipelineResponse, DialogType>('RailReceipt')

const DataTableRowActions = createRowActions<RailReceiptPipelineResponse>({
  useEntity,
  disableEdit: true,
  getDetailPath: (row) => `/incoming/rail/${row.pipelineStatus === 'PENDING' ? row.id : (row.actionId ?? row.id)}`,
})

function getColumns(t: TFunction): ColumnDef<RailReceiptPipelineResponse>[] {
  return [
    textColumn<RailReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber')),
    dateColumn<RailReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<RailReceiptPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }),
    textColumn<RailReceiptPipelineResponse>('productName', t('common:table.product'), { primary: false }),
    textColumn<RailReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty'), { primary: false }),
    statusColumn<RailReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    textColumn<RailReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber'), { primary: false }),
    textColumn<RailReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty'), { primary: false }),
    actionsColumn<RailReceiptPipelineResponse>(DataTableRowActions),
  ]
}

const routeApi = getRouteApi('/_authenticated/incoming/rail/')
const detailRoute = getRouteApi('/_authenticated/incoming/rail/$id')
const globalFilterFn = createGlobalFilter<RailReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function RailReceiptTable({ data }: { data: RailReceiptPipelineResponse[] }) {
  return <EntityTable tableId="rail-receipt" data={data} getColumns={getColumns} routeApi={routeApi} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
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
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description="Rail Waybill" formId="rail-waybill-form" isSubmitting={form.formState.isSubmitting}>
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
  const waybillQuery = useTransportRailWaybillGet(id)
  const acceptanceQuery = useAcceptanceCompositeGet(id)
  const isLoading = waybillQuery.isLoading && acceptanceQuery.isLoading

  if (isLoading)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  if (acceptanceQuery.data?.data) {
    const doc = acceptanceQuery.data.data
    return (
      <DocumentDetailPage
        config={{ title: 'Acceptance Document', entityLabel: 'Acceptance', backTo: '/incoming/rail', executeFn: acceptanceDocumentExecute, revertFn: acceptanceDocumentRevert, queryKey: railReceiptPipelineQueryQueryKey(), statusColorMap: documentStatusColors }}
        document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
        subtitle={t('common:nav.railReceipt')}
        relatedContent={(() => {
          const docs: RelatedDocument[] = []
          if (doc.railWaybillId) {
            docs.push({ type: 'basis', label: 'Rail Waybill', documentNumber: doc.railWaybillIdName ?? doc.railWaybillId, status: 'Pending', statusColorMap: documentStatusColors, to: `/incoming/rail/${doc.railWaybillId}` })
          }
          return <RelatedDocuments documents={docs} />
        })()}
        formContent={(
          <div className="grid grid-cols-3 gap-4">
            <div>
              <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
              <p>{doc.dateAccepted}</p>
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
              textColumn<AcceptanceItemResponse>('contractorIdName', t('common:table.contractor')),
              textColumn<AcceptanceItemResponse>('acceptedAmount', t('common:table.quantity')),
            ]}
            isLocked={doc.status === 'POSTED'}
            sectionTitle={t('common:sections.acceptanceItems')}
          />
        )}
        metadataContent={doc.executedAt
          ? (
              <div className="text-sm">
                <span className="text-muted-foreground">{t('common:metadata.executedAt')}:</span>
                {' '}
                {doc.executedAt}
              </div>
            )
          : null}
      />
    )
  }

  if (waybillQuery.data?.data) {
    const wb = waybillQuery.data.data
    return (
      <div className="mx-auto max-w-4xl space-y-6 p-4">
        <div className="flex items-center gap-3">
          <h1 className="text-lg font-semibold">
            Rail Waybill
            <span className="text-muted-foreground">{wb.documentNumber}</span>
          </h1>
          <span className="rounded-full bg-amber-100/30 px-3 py-1 text-xs text-amber-800 dark:bg-amber-900/30 dark:text-amber-300">Pending Acceptance</span>
        </div>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{wb.date}</p>
          </div>
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span>
            <p>{wb.senderIdName ?? wb.senderId}</p>
          </div>
        </div>
      </div>
    )
  }

  return <div className="p-4">Document not found</div>
}
