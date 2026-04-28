import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { StorageResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, resolvedColumn, StatusBadge, textColumn } from '~/components/data-table'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { CheckboxField, TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogStorageCreate, catalogStorageHardDelete, catalogStorageSoftDelete, catalogStorageUpdate } from '~/generated/client'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { catalogStorageListQueryKey, useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { entityActiveColors } from '~/lib/badge-colors'
import { defineCrudViews } from '~/lib/define-crud-views'
import { ProductTypeMutateDialog } from '~/views/catalog/product-types'
import { WarehouseMutateDialog } from '~/views/catalog/warehouses'

// --- Columns ---

function getStorageColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<StorageResponse> }>,
): ColumnDef<StorageResponse>[] {
  return [
    textColumn<StorageResponse>('commonName', t('entities:commonName'), { sizing: 'capped', maxSize: 180 }),
    resolvedColumn<StorageResponse>('warehouseId', t('entities:warehouse'), 'warehouseIdName'),
    numericColumn<StorageResponse>('capacity', t('entities:capacity')),
    resolvedColumn<StorageResponse>('productTypeId', t('entities:productType'), 'productTypeIdName'),
    {
      accessorKey: 'isTypeSpecific',
      minSize: 90,
      maxSize: 130,
      header: t('entities:isTypeSpecific'),
      meta: { sizingCategory: 'capped' as const, align: 'right' as const },
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
    actionsColumn<StorageResponse>(RowActions, 2),
  ]
}

// --- Table ---

const storagesRoute = getRouteApi('/_authenticated/catalog/storages/')
const storagesGlobalFilterFn = createGlobalFilter<StorageResponse>('commonName')

interface StoragesTableProps {
  data: StorageResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<StorageResponse> }>
}

function StoragesTable({ data, actions, RowActions }: StoragesTableProps) {
  return (
    <EntityTable
      tableId="storages"
      data={data}
      getColumns={t => getStorageColumns(t, RowActions)}
      routeApi={storagesRoute}
      globalFilterFn={storagesGlobalFilterFn}
      i18nNamespaces={['catalog', 'entities', 'common']}
      actions={actions}
    />
  )
}

function useStoragesTitle() {
  return useTranslation(['catalog']).t('catalog:storage.title')
}

// --- Mutate Dialog ---

const storageFormSchema = z.object({
  commonName: z.string().min(1),
  warehouseId: z.string().min(1),
  capacity: z.string().nullable().optional(),
  productTypeId: z.string().nullable().optional(),
  isTypeSpecific: z.boolean(),
})

type StorageFormValues = z.infer<typeof storageFormSchema>

interface StorageMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: StorageResponse | null
  onCreated?: (id: string) => void
}

function StorageMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: StorageMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'entities', 'common', 'forms'])

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
    onCreated,
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
          <TextField<StorageFormValues> name="commonName" label={t('entities:commonName')} />
          <EntityPickerField<StorageFormValues>
            name="warehouseId"
            label={t('entities:warehouse')}
            placeholder={t('forms:picker.selectEntity', { entity: t('entities:warehouse').toLowerCase() })}
            queryResult={warehousesQuery}
            displayField="commonName"
            allowCreate
            createDialog={WarehouseMutateDialog}
          />
          <TextField<StorageFormValues> name="capacity" label={t('entities:capacity')} nullable />
          <EntityPickerField<StorageFormValues>
            name="productTypeId"
            label={t('entities:productType')}
            placeholder={t('forms:picker.selectEntity', { entity: t('entities:productType').toLowerCase() })}
            queryResult={productTypesQuery}
            displayField="commonName"
            nullable
            allowCreate
            createDialog={ProductTypeMutateDialog}
          />
          <CheckboxField<StorageFormValues> name="isTypeSpecific" label={t('entities:isTypeSpecific')} />
        </form>
      </Form>
    </FormDialog>
  )
}

const storagesViewDefinition = defineCrudViews<StorageResponse>({
  displayName: 'Storages',
  useTitle: useStoragesTitle,
  useQuery: () => useCatalogStorageList({ embed: 'names' }),
  Table: StoragesTable,
  MutateDialog: StorageMutateDialog,
  deleteDialog: {
    hardDeleteFn: catalogStorageHardDelete,
    softDeleteFn: catalogStorageSoftDelete,
    queryKey: catalogStorageListQueryKey,
    entityLabel: 'catalog:storage.singular',
    i18nNamespaces: ['common', 'catalog'],
  },
})

export function Storages() {
  return <storagesViewDefinition.View />
}
