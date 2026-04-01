import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { WarehouseResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { BaseMutateDialog } from '~/features/catalog/bases'
import { catalogWarehouseCreate, catalogWarehouseHardDelete, catalogWarehouseSoftDelete, catalogWarehouseUpdate } from '~/generated/client'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { catalogWarehouseListQueryKey, useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type WarehousesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: WarehousesProvider, useEntity: useWarehouses }
  = createEntityProvider<WarehouseResponse, WarehousesDialogType>('Warehouses')

// --- Row Actions ---

const DataTableRowActions = createRowActions<WarehouseResponse>({ useEntity: useWarehouses })

// --- Columns ---

function getWarehouseColumns(t: TFunction): ColumnDef<WarehouseResponse>[] {
  return [
    selectColumn<WarehouseResponse>(),
    textColumn<WarehouseResponse>('commonName', t('catalog:warehouse.columns.commonName')),
    resolvedColumn<WarehouseResponse>('baseId', t('catalog:warehouse.columns.baseId'), 'baseIdName'),
    dateColumn<WarehouseResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<WarehouseResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const warehousesRoute = getRouteApi('/_authenticated/catalog/warehouses/')
const warehousesGlobalFilterFn = createGlobalFilter<WarehouseResponse>('commonName', 'longName')

interface WarehousesTableProps {
  data: WarehouseResponse[]
}

function WarehousesTable({ data }: WarehousesTableProps) {
  return (
    <EntityTable
      tableId="warehouses"
      data={data}
      getColumns={getWarehouseColumns}
      routeApi={warehousesRoute}
      globalFilterFn={warehousesGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      bulkActions={t => [
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

const warehouseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  baseId: z.string().min(1, 'Base is required'),
})

type WarehouseFormValues = z.infer<typeof warehouseFormSchema>

interface WarehouseMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: WarehouseResponse | null
  onCreated?: (id: string) => void
}

export function WarehouseMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: WarehouseMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const basesQuery = useCatalogBaseList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: warehouseFormSchema,
    defaultValues: {
      commonName: '',
      baseId: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      baseId: row.baseId,
    }),
    createFn: catalogWarehouseCreate,
    updateFn: catalogWarehouseUpdate,
    queryKey: catalogWarehouseListQueryKey(),
    entityLabel: t('catalog:warehouse.singular'),
    formId: 'warehouse-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      description={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      formId="warehouse-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="warehouse-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<WarehouseFormValues> name="commonName" label={t('catalog:warehouse.form.commonName')} />
          <EntityPickerField<WarehouseFormValues>
            name="baseId"
            label={t('catalog:warehouse.form.baseId')}
            placeholder={t('catalog:warehouse.form.selectBase')}
            queryResult={basesQuery}
            displayField="commonName"
            allowCreate
            createDialog={BaseMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const WarehouseDeleteDialog = createDeleteDialog({
  useEntity: useWarehouses,
  hardDeleteFn: catalogWarehouseHardDelete,
  softDeleteFn: catalogWarehouseSoftDelete,
  queryKey: catalogWarehouseListQueryKey,
  entityLabel: 'catalog:warehouse.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const WarehousesDialogs = createEntityDialogs({
  useEntity: useWarehouses,
  MutateDialog: WarehouseMutateDialog,
  DeleteDialog: WarehouseDeleteDialog,
})

// --- Primary Buttons ---

const WarehousesPrimaryButtons = createPrimaryButtons({
  useEntity: useWarehouses,
  createLabel: 'catalog:warehouse.create',
  i18nNamespaces: ['catalog'],
})

// --- Page ---

export function Warehouses() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogWarehouseList()

  return (
    <EntityPage
      provider={WarehousesProvider}
      title={t('catalog:warehouse.title')}
      queryResult={queryResult}
      primaryButtons={WarehousesPrimaryButtons}
      table={WarehousesTable}
      dialogs={WarehousesDialogs}
    />
  )
}
