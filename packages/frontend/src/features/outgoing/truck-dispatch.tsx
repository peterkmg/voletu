import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchItemResponse, TruckDispatchPipelineResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentRevert } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { truckDispatchPipelineQueryQueryKey, useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<TruckDispatchPipelineResponse, DialogType>('TruckDispatch')

const DataTableRowActions = createRowActions<TruckDispatchPipelineResponse>({
  useEntity,
  disableEdit: true,
  getDetailPath: (row) => `/outgoing/truck/${row.id}`,
})

function getColumns(t: TFunction): ColumnDef<TruckDispatchPipelineResponse>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<TruckDispatchPipelineResponse>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<TruckDispatchPipelineResponse>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<TruckDispatchPipelineResponse>('contractorName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...statusColumn<TruckDispatchPipelineResponse>('pipelineStatus', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<TruckDispatchPipelineResponse>('productName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<TruckDispatchPipelineResponse>('dispatchedQuantity', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    // Actions (doc-level)
    { ...actionsColumn<TruckDispatchPipelineResponse>(DataTableRowActions, 1), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const routeApi = getRouteApi('/_authenticated/outgoing/truck/')
const detailRoute = getRouteApi('/_authenticated/outgoing/truck/$id')
const globalFilterFn = createGlobalFilter<TruckDispatchPipelineResponse>('documentNumber', 'contractorName')

function TruckDispatchTable({ data }: { data: TruckDispatchPipelineResponse[] }) {
  return (
    <EntityTable
      tableId="truck-dispatch"
      data={data}
      getColumns={getColumns}
      routeApi={routeApi}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="id"
    />
  )
}

const dispatchSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  contractorId: z.string().uuid(),
})

type DispatchFormValues = z.infer<typeof dispatchSchema>

function DispatchMutateDialog({ open, onOpenChange }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: TruckDispatchPipelineResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    schema: dispatchSchema,
    defaultValues: { documentNumber: '', date: '', contractorId: '' },
    transformPayload: v => ({ ...v, dispatchMethod: 'TRUCK' as const, dispatchPurpose: 'EXTERNAL' as const }),
    createFn: dispatchDocumentCreate,
    queryKey: truckDispatchPipelineQueryQueryKey(),
    entityLabel: t('common:nav.truckDispatch'),
    formId: 'truck-dispatch-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description="Truck Dispatch" formId="truck-dispatch-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="truck-dispatch-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<DispatchFormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<DispatchFormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<DispatchFormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog: DispatchMutateDialog })

const PrimaryButtons = createPrimaryButtons({ useEntity })

export function TruckDispatchPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useTruckDispatchPipelineQuery()

  return <EntityPage provider={Provider} title={t('common:nav.truckDispatch')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={TruckDispatchTable} dialogs={Dialogs} />
}

export function TruckDispatchDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useDispatchCompositeGet(id, { embed: 'names' })

  if (isLoading || !data?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.truckDispatch'), entityLabel: 'Dispatch', backTo: '/outgoing/truck', executeFn: dispatchDocumentExecute, revertFn: dispatchDocumentRevert, queryKey: truckDispatchPipelineQueryQueryKey(), statusColorMap: statusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{formatDate(doc.date)}</p>
          </div>
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span>
            <p>{doc.contractorIdName ?? doc.contractorId}</p>
          </div>
        </div>
      )}
      itemsContent={(
        <ChildItemsTable
          items={doc.items}
          columns={[
            textColumn<DispatchItemResponse>('productIdName', t('common:table.product')),
            textColumn<DispatchItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<DispatchItemResponse>('dispatchedAmount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'EXECUTED'}
          sectionTitle={t('common:sections.dispatchItems')}
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
