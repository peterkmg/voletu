import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferItemResponse, PhysicalTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { Skeleton } from '~/components/ui/skeleton'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { physicalDocumentCreate, physicalDocumentExecute, physicalDocumentHardDelete, physicalDocumentRevert, physicalDocumentSoftDelete, physicalDocumentUpdate } from '~/generated/client'
import { usePhysicalTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferCompositeGet'
import { physicalTransferListQueryKey, usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<PhysicalTransferResponse, DialogType>('PhysicalTransfer')

const DataTableRowActions = createRowActions<PhysicalTransferResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<PhysicalTransferResponse>[] {
  return [
    selectColumn<PhysicalTransferResponse>(),
    textColumn<PhysicalTransferResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<PhysicalTransferResponse>('date', t('common:table.date')),
    statusColumn<PhysicalTransferResponse>('status', t('common:table.status'), documentStatusColors),
    actionsColumn<PhysicalTransferResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/internal/physical-transfer/')
const detailRoute = getRouteApi('/_authenticated/internal/physical-transfer/$id')
const globalFilterFn = createGlobalFilter<PhysicalTransferResponse>('documentNumber')

function PhysicalTransferTable({ data }: { data: PhysicalTransferResponse[] }) {
  return <EntityTable tableId="physical-transfer" data={data} getColumns={getColumns} routeApi={route} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: PhysicalTransferResponse | null }) {
  const { t } = useTranslation(['common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, currentRow, schema: formSchema,
    defaultValues: { documentNumber: '', date: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '' }),
    createFn: physicalDocumentCreate, updateFn: physicalDocumentUpdate,
    queryKey: physicalTransferListQueryKey(), entityLabel: t('common:nav.physicalTransfer'), formId: 'physical-transfer-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.physicalTransfer')} formId="physical-transfer-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="physical-transfer-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: physicalDocumentHardDelete, softDeleteFn: physicalDocumentSoftDelete, queryKey: physicalTransferListQueryKey, entityLabel: 'common:nav.physicalTransfer', i18nNamespaces: ['common'] })

function PhysicalLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: PhysicalTransferResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={physicalDocumentExecute} revertFn={physicalDocumentRevert} queryKey={physicalTransferListQueryKey()} entityLabel="Physical Transfer" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: PhysicalLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function PhysicalTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = usePhysicalTransferList()

  return <EntityPage provider={Provider} title={t('common:nav.physicalTransfer')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={PhysicalTransferTable} dialogs={Dialogs} />
}

export function PhysicalTransferDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = usePhysicalTransferCompositeGet(id)

  if (isLoading || !data?.data) return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data
  const items = doc.items ?? []

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.physicalTransfer'), entityLabel: 'Physical Transfer', backTo: '/internal/physical-transfer', executeFn: physicalDocumentExecute, revertFn: physicalDocumentRevert, queryKey: physicalTransferListQueryKey(), statusColorMap: documentStatusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={
        <div className="grid grid-cols-3 gap-4">
          <div><span className="text-sm text-muted-foreground">{t('common:table.date')}</span><p>{doc.date}</p></div>
        </div>
      }
      itemsContent={
        <ChildItemsTable
          items={items}
          columns={[
            textColumn<PhysicalTransferItemResponse>('contractorIdName', t('common:table.contractor')),
            textColumn<PhysicalTransferItemResponse>('productIdName', t('common:table.product')),
            textColumn<PhysicalTransferItemResponse>('fromStorageIdName', 'From Storage'),
            textColumn<PhysicalTransferItemResponse>('toStorageIdName', 'To Storage'),
            textColumn<PhysicalTransferItemResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'POSTED'}
          sectionTitle="Transfer Items"
        />
      }
      metadataContent={doc.executedAt ? <div className="text-sm"><span className="text-muted-foreground">Executed at:</span> {doc.executedAt}</div> : null}
    />
  )
}
