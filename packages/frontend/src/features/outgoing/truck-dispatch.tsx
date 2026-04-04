import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchItemResponse, TruckDispatchPipelineResponse } from '~/generated/types'
import { getRouteApi, useNavigate } from '@tanstack/react-router'
import { ChevronRight } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { RowActions } from '~/components/data-table/row-actions'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Button } from '~/components/ui/button'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentRevert } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { truckDispatchPipelineQueryQueryKey, useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors, pipelineStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<TruckDispatchPipelineResponse, DialogType>('TruckDispatch')

function DataTableRowActions({ row }: { row: { original: TruckDispatchPipelineResponse } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <RowActions
      actions={[
        {
          label: t('actions.viewDetails'),
          icon: ChevronRight,
          inline: true,
          onClick: () => navigate({ to: `/outgoing/truck/${row.original.id}` }),
        },
      ]}
    />
  )
}

function getColumns(t: TFunction): ColumnDef<TruckDispatchPipelineResponse>[] {
  return [
    selectColumn<TruckDispatchPipelineResponse>(),
    textColumn<TruckDispatchPipelineResponse>('documentNumber', t('common:table.documentNumber')),
    textColumn<TruckDispatchPipelineResponse>('date', t('common:table.date')),
    textColumn<TruckDispatchPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<TruckDispatchPipelineResponse>('productName', t('common:table.product')),
    textColumn<TruckDispatchPipelineResponse>('dispatchedQuantity', t('common:table.quantity')),
    statusColumn<TruckDispatchPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    actionsColumn<TruckDispatchPipelineResponse>(DataTableRowActions),
  ]
}

const routeApi = getRouteApi('/_authenticated/outgoing/truck/')
const detailRoute = getRouteApi('/_authenticated/outgoing/truck/$id')
const globalFilterFn = createGlobalFilter<TruckDispatchPipelineResponse>('documentNumber', 'contractorName')

function TruckDispatchTable({ data }: { data: TruckDispatchPipelineResponse[] }) {
  return <EntityTable tableId="truck-dispatch" data={data} getColumns={getColumns} routeApi={routeApi} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
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

function PrimaryButtons() {
  const { t } = useTranslation('common')
  const { setOpen, setCurrentRow } = useEntity()

  return (
    <Button size="sm" onClick={() => { setCurrentRow(null); setOpen('create') }}>
      {t('actions.create')}
      {' '}
      Dispatch
    </Button>
  )
}

export function TruckDispatchPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useTruckDispatchPipelineQuery()

  return <EntityPage provider={Provider} title={t('common:nav.truckDispatch')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={TruckDispatchTable} dialogs={Dialogs} />
}

export function TruckDispatchDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useDispatchCompositeGet(id)

  if (isLoading || !data?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.truckDispatch'), entityLabel: 'Dispatch', backTo: '/outgoing/truck', executeFn: dispatchDocumentExecute, revertFn: dispatchDocumentRevert, queryKey: truckDispatchPipelineQueryQueryKey(), statusColorMap: documentStatusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{doc.date}</p>
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
            textColumn<DispatchItemResponse>('storageIdName', 'Storage'),
            textColumn<DispatchItemResponse>('dispatchedAmount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'POSTED'}
          sectionTitle="Dispatch Items"
        />
      )}
      metadataContent={doc.executedAt
        ? (
            <div className="text-sm">
              <span className="text-muted-foreground">Executed at:</span>
              {' '}
              {doc.executedAt}
            </div>
          )
        : null}
    />
  )
}
