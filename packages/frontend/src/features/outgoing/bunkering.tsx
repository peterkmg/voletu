import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchItemResponse, DispatchResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { Skeleton } from '~/components/ui/skeleton'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentHardDelete, dispatchDocumentRevert, dispatchDocumentSoftDelete, dispatchDocumentUpdate } from '~/generated/client'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { dispatchDocumentQueryQueryKey, useDispatchDocumentQuery } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<DispatchResponse, DialogType>('Bunkering')

const DataTableRowActions = createRowActions<DispatchResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<DispatchResponse>[] {
  return [
    selectColumn<DispatchResponse>(),
    textColumn<DispatchResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<DispatchResponse>('date', t('common:table.date')),
    resolvedColumn<DispatchResponse>('contractorId', t('common:table.contractor'), 'contractorIdName'),
    statusColumn<DispatchResponse>('status', t('common:table.status'), documentStatusColors),
    { ...dateColumn<DispatchResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), requiresRole: 'senior_supervisor' } },
    actionsColumn<DispatchResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/outgoing/bunkering/')
const detailRoute = getRouteApi('/_authenticated/outgoing/bunkering/$id')
const globalFilterFn = createGlobalFilter<DispatchResponse>('documentNumber')

function BunkeringTable({ data }: { data: DispatchResponse[] }) {
  return <EntityTable tableId="bunkering" data={data} getColumns={getColumns} routeApi={route} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  contractorId: z.string().uuid(),
  bunkerType: z.enum(['DOMESTIC', 'EXPORT']),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: DispatchResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, currentRow, schema: formSchema,
    defaultValues: { documentNumber: '', date: '', contractorId: '', bunkerType: 'DOMESTIC' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '', contractorId: row.contractorId, bunkerType: row.bunkerType ?? 'DOMESTIC' }),
    transformPayload: v => ({ ...v, dispatchMethod: 'BUNKERING' as const, dispatchPurpose: 'EXTERNAL' as const, bunkerType: v.bunkerType as 'DOMESTIC' | 'EXPORT' }),
    createFn: dispatchDocumentCreate, updateFn: dispatchDocumentUpdate,
    queryKey: dispatchDocumentQueryQueryKey(), entityLabel: t('common:nav.bunkering'), formId: 'bunkering-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.bunkering')} formId="bunkering-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="bunkering-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<FormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
          <TextField<FormValues> name="bunkerType" label="Bunker Type" />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: dispatchDocumentHardDelete, softDeleteFn: dispatchDocumentSoftDelete, queryKey: dispatchDocumentQueryQueryKey, entityLabel: 'common:nav.bunkering', i18nNamespaces: ['common'] })

function BunkeringLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: DispatchResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={dispatchDocumentExecute} revertFn={dispatchDocumentRevert} queryKey={dispatchDocumentQueryQueryKey()} entityLabel="Bunkering Document" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: BunkeringLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function BunkeringPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useDispatchDocumentQuery({ dispatchMethod: 'BUNKERING' as any })

  return <EntityPage provider={Provider} title={t('common:nav.bunkering')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={BunkeringTable} dialogs={Dialogs} />
}

export function BunkeringDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useDispatchCompositeGet(id)

  if (isLoading || !data?.data) return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.bunkering'), entityLabel: 'Bunkering', backTo: '/outgoing/bunkering', executeFn: dispatchDocumentExecute, revertFn: dispatchDocumentRevert, queryKey: dispatchDocumentQueryQueryKey(), statusColorMap: documentStatusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={
        <div className="grid grid-cols-3 gap-4">
          <div><span className="text-sm text-muted-foreground">{t('common:table.date')}</span><p>{doc.date}</p></div>
          <div><span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span><p>{doc.contractorIdName ?? doc.contractorId}</p></div>
          <div><span className="text-sm text-muted-foreground">Bunker Type</span><p>{doc.bunkerType ?? '—'}</p></div>
        </div>
      }
      itemsContent={
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
      }
      metadataContent={doc.executedAt ? <div className="text-sm"><span className="text-muted-foreground">Executed at:</span> {doc.executedAt}</div> : null}
    />
  )
}
