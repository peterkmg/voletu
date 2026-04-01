import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { InventoryReconciliationResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, DataTableColumnHeader, dateColumn, EntityTable, LookupCell, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPickerField } from '~/components/entity-picker'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { WarehouseMutateDialog } from '~/features/catalog/warehouses'
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

// --- Provider ---

type ReconciliationDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: ReconciliationProvider, useEntity: useReconciliation }
  = createEntityProvider<InventoryReconciliationResponse, ReconciliationDialogType>('Reconciliation')

// --- Row Actions ---

const DataTableRowActions = createRowActions<InventoryReconciliationResponse>({ useEntity: useReconciliation, lifecycle: true })

// --- Columns ---

interface ReconciliationColumnLookups {
  warehouseMap: Map<string, string>
}

function getReconciliationColumns(t: TFunction, lookups: ReconciliationColumnLookups): ColumnDef<InventoryReconciliationResponse>[] {
  return [
    selectColumn<InventoryReconciliationResponse>(),
    textColumn<InventoryReconciliationResponse>('documentNumber', t('documents:reconciliation.columns.documentNumber')),
    dateColumn<InventoryReconciliationResponse>('date', t('documents:reconciliation.columns.date')),
    {
      accessorKey: 'warehouseId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:nav.warehouses')}
        />
      ),
      cell: ({ row }) => (
        <LookupCell value={row.getValue('warehouseId')} lookupMap={lookups.warehouseMap} />
      ),
    },
    statusColumn<InventoryReconciliationResponse>('status', t('documents:reconciliation.columns.status'), documentStatusColors),
    dateColumn<InventoryReconciliationResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<InventoryReconciliationResponse>(DataTableRowActions),
  ]
}

// --- Global Filter + Table ---

const route = getRouteApi('/_authenticated/documents/inventory-reconciliation/')
const globalFilterFn = createGlobalFilter<InventoryReconciliationResponse>('documentNumber')

interface ReconciliationTableProps {
  data: InventoryReconciliationResponse[]
}

function ReconciliationTable({ data }: ReconciliationTableProps) {
  const { data: warehousesData } = useCatalogWarehouseList()
  const warehouseMap = useMemo(() => {
    const map = new Map<string, string>()
    for (const w of warehousesData?.data ?? []) map.set(w.id, w.commonName)
    return map
  }, [warehousesData])

  const getColumns = useCallback(
    (t: Parameters<typeof getReconciliationColumns>[0]) => getReconciliationColumns(t, { warehouseMap }),
    [warehouseMap],
  )

  return (
    <EntityTable
      tableId="reconciliation"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['documents', 'common']}
      bulkActions={t => [
        {
          label: t('common:actions.execute'),
          icon: Play,
          onClick: (rows) => {
            const draftRows = rows.filter(r => r.status === 'DRAFT')
            void draftRows // TODO: wire bulk execute API
          },
        },
        {
          label: t('common:actions.softDelete'),
          icon: Archive,
          variant: 'destructive',
          onClick: (rows) => {
            void rows // TODO: wire bulk soft-delete API
          },
        },
      ]}
    />
  )
}

// --- Mutate Dialog ---

const reconciliationFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  warehouseId: z.string().min(1, 'Warehouse is required'),
})

type ReconciliationFormValues = z.infer<typeof reconciliationFormSchema>

interface ReconciliationMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: InventoryReconciliationResponse | null
}

function ReconciliationMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: ReconciliationMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const warehousesQuery = useCatalogWarehouseList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: reconciliationFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      warehouseId: '',
    },
    mapRowToForm: (row: InventoryReconciliationResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date ? row.date.slice(0, 16) : '',
      warehouseId: row.warehouseId,
    }),
    createFn: reconciliationCreate,
    updateFn: reconciliationUpdate,
    queryKey: reconciliationListQueryKey(),
    entityLabel: t('documents:reconciliation.singular'),
    formId: 'reconciliation-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('documents:reconciliation.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:reconciliation.create')}
      formId="reconciliation-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="reconciliation-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<ReconciliationFormValues> name="documentNumber" label={t('documents:reconciliation.columns.documentNumber')} />
          <TextField<ReconciliationFormValues> name="date" label={t('documents:reconciliation.columns.date')} type="datetime-local" />
          <EntityPickerField<ReconciliationFormValues>
            name="warehouseId"
            label={t('common:nav.warehouses')}
            queryResult={warehousesQuery}
            displayField="commonName"
            allowCreate
            createDialog={WarehouseMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const ReconciliationDeleteDialog = createDeleteDialog({
  useEntity: useReconciliation,
  hardDeleteFn: reconciliationHardDelete,
  softDeleteFn: reconciliationSoftDelete,
  queryKey: reconciliationListQueryKey,
  entityLabel: 'documents:reconciliation.singular',
  i18nNamespaces: ['common', 'documents'],
})

// --- Lifecycle Dialog ---

interface ReconciliationLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: InventoryReconciliationResponse | null
  variant: 'execute' | 'revert'
}

function ReconciliationLifecycleDialog({ open, onOpenChange, currentRow, variant }: ReconciliationLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={reconciliationExecute}
      revertFn={reconciliationRevert}
      queryKey={reconciliationListQueryKey()}
      entityLabel={t('documents:reconciliation.singular')}
    />
  )
}

// --- Entity Dialogs ---

const ReconciliationDialogs = createEntityDialogs({
  useEntity: useReconciliation,
  MutateDialog: ReconciliationMutateDialog,
  DeleteDialog: ReconciliationDeleteDialog,
  LifecycleDialog: ReconciliationLifecycleDialog,
  lifecyclePropName: 'variant',
})

// --- Primary Buttons ---

const ReconciliationPrimaryButtons = createPrimaryButtons({
  useEntity: useReconciliation,
  createLabel: 'documents:reconciliation.create',
  i18nNamespaces: ['documents'],
})

// --- Page Component ---

export function InventoryReconciliation() {
  const { t } = useTranslation(['documents'])
  const queryResult = useReconciliationList()

  return (
    <EntityPage
      provider={ReconciliationProvider}
      title={t('documents:reconciliation.title')}
      queryResult={queryResult}
      primaryButtons={ReconciliationPrimaryButtons}
      table={ReconciliationTable}
      dialogs={ReconciliationDialogs}
    />
  )
}
