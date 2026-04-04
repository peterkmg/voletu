import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferItemResponse, OwnershipTransferResponse } from '~/generated/types'
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
import { ownershipDocumentCreate, ownershipDocumentExecute, ownershipDocumentHardDelete, ownershipDocumentRevert, ownershipDocumentSoftDelete, ownershipDocumentUpdate } from '~/generated/client'
import { useOwnershipTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferCompositeGet'
import { ownershipTransferListQueryKey, useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<OwnershipTransferResponse, DialogType>('OwnershipTransfer')

const DataTableRowActions = createRowActions<OwnershipTransferResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<OwnershipTransferResponse>[] {
  return [
    selectColumn<OwnershipTransferResponse>(),
    dateColumn<OwnershipTransferResponse>('date', t('common:table.date')),
    statusColumn<OwnershipTransferResponse>('status', t('common:table.status'), documentStatusColors),
    { ...dateColumn<OwnershipTransferResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), requiresRole: 'senior_supervisor' } },
    actionsColumn<OwnershipTransferResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/internal/ownership-transfer/')
const detailRoute = getRouteApi('/_authenticated/internal/ownership-transfer/$id')
const globalFilterFn = createGlobalFilter<OwnershipTransferResponse>('id')

function OwnershipTransferTable({ data }: { data: OwnershipTransferResponse[] }) {
  return <EntityTable tableId="ownership-transfer" data={data} getColumns={getColumns} routeApi={route} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const formSchema = z.object({
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: OwnershipTransferResponse | null }) {
  const { t } = useTranslation(['common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, currentRow, schema: formSchema,
    defaultValues: { date: '' },
    mapRowToForm: row => ({ date: row.date?.split('T')[0] ?? '' }),
    createFn: ownershipDocumentCreate, updateFn: ownershipDocumentUpdate,
    queryKey: ownershipTransferListQueryKey(), entityLabel: t('common:nav.ownershipTransfer'), formId: 'ownership-transfer-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.ownershipTransfer')} formId="ownership-transfer-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="ownership-transfer-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: ownershipDocumentHardDelete, softDeleteFn: ownershipDocumentSoftDelete, queryKey: ownershipTransferListQueryKey, entityLabel: 'common:nav.ownershipTransfer', i18nNamespaces: ['common'] })

function OwnershipLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: OwnershipTransferResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={ownershipDocumentExecute} revertFn={ownershipDocumentRevert} queryKey={ownershipTransferListQueryKey()} entityLabel="Ownership Transfer" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: OwnershipLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function OwnershipTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useOwnershipTransferList()

  return <EntityPage provider={Provider} title={t('common:nav.ownershipTransfer')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={OwnershipTransferTable} dialogs={Dialogs} />
}

export function OwnershipTransferDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useOwnershipTransferCompositeGet(id)

  if (isLoading || !data?.data) return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data
  const items = doc.items ?? []

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.ownershipTransfer'), entityLabel: 'Ownership Transfer', backTo: '/internal/ownership-transfer', executeFn: ownershipDocumentExecute, revertFn: ownershipDocumentRevert, queryKey: ownershipTransferListQueryKey(), statusColorMap: documentStatusColors }}
      document={{ id: doc.id, documentNumber: doc.id, status: doc.status }}
      formContent={
        <div className="grid grid-cols-3 gap-4">
          <div><span className="text-sm text-muted-foreground">{t('common:table.date')}</span><p>{doc.date}</p></div>
        </div>
      }
      itemsContent={
        <ChildItemsTable
          items={items}
          columns={[
            textColumn<OwnershipTransferItemResponse>('fromContractorIdName', 'From Contractor'),
            textColumn<OwnershipTransferItemResponse>('toContractorIdName', 'To Contractor'),
            textColumn<OwnershipTransferItemResponse>('productIdName', t('common:table.product')),
            textColumn<OwnershipTransferItemResponse>('storageIdName', 'Storage'),
            textColumn<OwnershipTransferItemResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'POSTED'}
          sectionTitle="Transfer Items"
        />
      }
      metadataContent={doc.executedAt ? <div className="text-sm"><span className="text-muted-foreground">Executed at:</span> {doc.executedAt}</div> : null}
    />
  )
}
