import type { StorageResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { CheckboxField, TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { ProductTypeMutateDialog } from '~/features/catalog/product-types/components/product-type-mutate-dialog'
import { WarehouseMutateDialog } from '~/features/catalog/warehouses/components/warehouse-mutate-dialog'
import { catalogStorageCreate, catalogStorageUpdate } from '~/generated/client'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { catalogStorageListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

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

export function StorageMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: StorageMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const warehousesQuery = useCatalogWarehouseList()
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
