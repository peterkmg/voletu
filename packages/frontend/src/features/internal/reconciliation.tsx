import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { InventoryReconciliationResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { reconciliationCreate, reconciliationExecute, reconciliationHardDelete, reconciliationRevert, reconciliationSoftDelete, reconciliationUpdate } from '~/generated/client'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
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

const DataTableRowActions = createRowActions<InventoryReconciliationResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<InventoryReconciliationResponse>[] {
  return [
    selectColumn<InventoryReconciliationResponse>(),
    textColumn<InventoryReconciliationResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<InventoryReconciliationResponse>('date', t('common:table.date')),
    statusColumn<InventoryReconciliationResponse>('status', t('common:table.status'), documentStatusColors),
    actionsColumn<InventoryReconciliationResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/internal/reconciliation/')
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
    open, onOpenChange, currentRow, schema: formSchema,
    defaultValues: { documentNumber: '', date: '', warehouseId: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '', warehouseId: row.warehouseId }),
    createFn: reconciliationCreate, updateFn: reconciliationUpdate,
    queryKey: reconciliationListQueryKey(), entityLabel: t('common:nav.reconciliation'), formId: 'reconciliation-form',
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
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function ReconciliationPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useReconciliationList()

  return <EntityPage provider={Provider} title={t('common:nav.reconciliation')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={ReconciliationTable} dialogs={Dialogs} />
}

export function ReconciliationDetail() {
  return <div className="p-4">Reconciliation Detail — TODO</div>
}
