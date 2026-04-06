import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { StorageResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, resolvedColumn, StatusBadge, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { CheckboxField, TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { ProductTypeMutateDialog } from '~/features/catalog/product-types'
import { WarehouseMutateDialog } from '~/features/catalog/warehouses'
import { catalogStorageCreate, catalogStorageHardDelete, catalogStorageSoftDelete, catalogStorageUpdate } from '~/generated/client'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { catalogStorageListQueryKey, useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { entityActiveColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type StoragesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: StoragesProvider, useEntity: useStorages }
  = createEntityProvider<StorageResponse, StoragesDialogType>('Storages')

// --- Row Actions ---

const DataTableRowActions = createRowActions<StorageResponse>({ useEntity: useStorages })

// --- Columns ---

function getStorageColumns(t: TFunction): ColumnDef<StorageResponse>[] {
  return [
    textColumn<StorageResponse>('commonName', t('catalog:storage.columns.commonName')),
    resolvedColumn<StorageResponse>('warehouseId', t('catalog:storage.columns.warehouseId'), 'warehouseIdName'),
    numericColumn<StorageResponse>('capacity', t('catalog:storage.columns.capacity')),
    resolvedColumn<StorageResponse>('productTypeId', t('catalog:storage.columns.productTypeId'), 'productTypeIdName'),
    {
      accessorKey: 'isTypeSpecific',
      header: t('catalog:storage.columns.isTypeSpecific'),
      cell: ({ row }) => {
        const value = row.getValue<boolean>('isTypeSpecific')
        return (
          <StatusBadge
            value={value ? 'active' : 'archived'}
            label={value ? t('common:yes') : t('common:no')}
            colorMap={entityActiveColors}
            className="text-xs"
          />
        )
      },
    },
    { ...dateColumn<StorageResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<StorageResponse>(DataTableRowActions, 2),
  ]
}

// --- Table ---

const storagesRoute = getRouteApi('/_authenticated/catalog/storages/')
const storagesGlobalFilterFn = createGlobalFilter<StorageResponse>('commonName')

interface StoragesTableProps {
  data: StorageResponse[]
}

function StoragesTable({ data }: StoragesTableProps) {
  return (
    <EntityTable
      tableId="storages"
      data={data}
      getColumns={getStorageColumns}
      routeApi={storagesRoute}
      globalFilterFn={storagesGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
    />
  )
}

// --- Mutate Dialog ---

const storageFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  warehouseId: z.string().min(1, 'Warehouse is required'),
  capacity: z.string().nullable().optional(),
  productTypeId: z.string().nullable().optional(),
  isTypeSpecific: z.boolean(),
})

type StorageFormValues = z.infer<typeof storageFormSchema>

interface StorageMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: StorageResponse | null
}

function StorageMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: StorageMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const warehousesQuery = useCatalogWarehouseList({ embed: 'names' })
  const productTypesQuery = useCatalogProductTypeList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: storageFormSchema,
    defaultValues: {
      commonName: '',
      warehouseId: '',
      capacity: '',
      productTypeId: '',
      isTypeSpecific: false,
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      warehouseId: row.warehouseId,
      capacity: row.capacity ?? '',
      productTypeId: row.productTypeId ?? '',
      isTypeSpecific: row.isTypeSpecific,
    }),
    transformPayload: values => ({
      ...values,
      capacity: values.capacity || null,
      productTypeId: values.productTypeId || null,
    }),
    createFn: catalogStorageCreate,
    updateFn: catalogStorageUpdate,
    queryKey: catalogStorageListQueryKey(),
    entityLabel: t('catalog:storage.singular'),
    formId: 'storage-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:storage.edit') : t('catalog:storage.create')}
      description={isUpdate ? t('catalog:storage.edit') : t('catalog:storage.create')}
      formId="storage-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="storage-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<StorageFormValues> name="commonName" label={t('catalog:storage.form.commonName')} />
          <EntityPickerField<StorageFormValues>
            name="warehouseId"
            label={t('catalog:storage.form.warehouseId')}
            placeholder={t('catalog:storage.form.selectWarehouse')}
            queryResult={warehousesQuery}
            displayField="commonName"
            allowCreate
            createDialog={WarehouseMutateDialog}
          />
          <TextField<StorageFormValues> name="capacity" label={t('catalog:storage.form.capacity')} nullable />
          <EntityPickerField<StorageFormValues>
            name="productTypeId"
            label={t('catalog:storage.form.productTypeId')}
            placeholder={t('catalog:storage.form.selectProductType')}
            queryResult={productTypesQuery}
            displayField="commonName"
            nullable
            allowCreate
            createDialog={ProductTypeMutateDialog}
          />
          <CheckboxField<StorageFormValues> name="isTypeSpecific" label={t('catalog:storage.form.isTypeSpecific')} />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const StorageDeleteDialog = createDeleteDialog({
  useEntity: useStorages,
  hardDeleteFn: catalogStorageHardDelete,
  softDeleteFn: catalogStorageSoftDelete,
  queryKey: catalogStorageListQueryKey,
  entityLabel: 'catalog:storage.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const StoragesDialogs = createEntityDialogs({
  useEntity: useStorages,
  MutateDialog: StorageMutateDialog,
  DeleteDialog: StorageDeleteDialog,
})

// --- Primary Buttons ---

const StoragesPrimaryButtons = createPrimaryButtons({ useEntity: useStorages })

// --- Page ---

export function Storages() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogStorageList({ embed: 'names' })

  return (
    <EntityPage
      provider={StoragesProvider}
      title={t('catalog:storage.title')}
      queryResult={queryResult}
      primaryButtons={StoragesPrimaryButtons}
      table={StoragesTable}
      dialogs={StoragesDialogs}
    />
  )
}
