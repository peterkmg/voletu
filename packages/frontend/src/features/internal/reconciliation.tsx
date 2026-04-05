import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { InventoryAdjustmentResponse, InventoryReconciliationResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { reconciliationCreate, reconciliationExecute, reconciliationHardDelete, reconciliationRevert, reconciliationSoftDelete, reconciliationUpdate } from '~/generated/client'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useAdjustmentList } from '~/generated/hooks/DocumentOperationsHooks/useAdjustmentList'
import { useReconciliationGet } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationGet'
import { reconciliationListQueryKey, useReconciliationList } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<InventoryReconciliationResponse, DialogType>('Reconciliation')

const DataTableRowActions = createRowActions<InventoryReconciliationResponse>({ useEntity, lifecycle: true, getDetailPath: row => `/internal/reconciliation/${row.id}` })

function getColumns(t: TFunction): ColumnDef<InventoryReconciliationResponse>[] {
  return [
    textColumn<InventoryReconciliationResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<InventoryReconciliationResponse>('date', t('common:table.date')),
    statusColumn<InventoryReconciliationResponse>('status', t('common:table.status'), documentStatusColors),
    { ...dateColumn<InventoryReconciliationResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), requiresRole: 'senior_supervisor' } },
    { ...dateColumn<InventoryReconciliationResponse>('updatedAt', t('common:table.updatedAt')), enableHiding: true, meta: { label: t('common:table.updatedAt'), requiresRole: 'senior_supervisor' } },
    actionsColumn<InventoryReconciliationResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/internal/reconciliation/')
const detailRoute = getRouteApi('/_authenticated/internal/reconciliation/$id')
const globalFilterFn = createGlobalFilter<InventoryReconciliationResponse>('documentNumber')

function ReconciliationTable({ data }: { data: InventoryReconciliationResponse[] }) {
  return <EntityTable tableId="reconciliation" data={data} getColumns={getColumns} routeApi={route} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  warehouseId: z.string().uuid(),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: InventoryReconciliationResponse | null }) {
  const { t } = useTranslation(['common'])
  const warehousesQuery = useCatalogWarehouseList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { documentNumber: '', date: '', warehouseId: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '', warehouseId: row.warehouseId }),
    createFn: reconciliationCreate,
    updateFn: reconciliationUpdate,
    queryKey: reconciliationListQueryKey(),
    entityLabel: t('common:nav.reconciliation'),
    formId: 'reconciliation-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.reconciliation')} formId="reconciliation-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="reconciliation-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<FormValues> name="warehouseId" label="Warehouse" queryResult={warehousesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: reconciliationHardDelete, softDeleteFn: reconciliationSoftDelete, queryKey: reconciliationListQueryKey, entityLabel: 'common:nav.reconciliation', i18nNamespaces: ['common'] })

function ReconciliationLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: InventoryReconciliationResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={reconciliationExecute} revertFn={reconciliationRevert} queryKey={reconciliationListQueryKey()} entityLabel="Reconciliation" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: ReconciliationLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity })

export function ReconciliationPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useReconciliationList()

  return <EntityPage provider={Provider} title={t('common:nav.reconciliation')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={ReconciliationTable} dialogs={Dialogs} />
}

export function ReconciliationDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data: docData, isLoading } = useReconciliationGet(id)
  const { data: itemsData } = useAdjustmentList()

  if (isLoading || !docData?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = docData.data
  const items = (itemsData?.data ?? []).filter((i: InventoryAdjustmentResponse) => i.reconciliationId === id)

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.reconciliation'), entityLabel: 'Reconciliation', backTo: '/internal/reconciliation', executeFn: reconciliationExecute, revertFn: reconciliationRevert, queryKey: reconciliationListQueryKey(), statusColorMap: documentStatusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{doc.date}</p>
          </div>
          <div>
            <span className="text-sm text-muted-foreground">{t('common:columns.warehouse')}</span>
            <p>{doc.warehouseIdName ?? doc.warehouseId}</p>
          </div>
        </div>
      )}
      itemsContent={(
        <ChildItemsTable
          items={items}
          columns={[
            textColumn<InventoryAdjustmentResponse>('productIdName', t('common:table.product')),
            textColumn<InventoryAdjustmentResponse>('storageIdName', t('common:columns.storage')),
            textColumn<InventoryAdjustmentResponse>('contractorIdName', t('common:table.contractor')),
            textColumn<InventoryAdjustmentResponse>('adjustmentType', t('common:columns.type')),
            textColumn<InventoryAdjustmentResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'POSTED'}
          sectionTitle={t('common:sections.adjustments')}
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
