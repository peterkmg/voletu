import type { StorageResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
import { toast } from 'sonner'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { Checkbox } from '~/components/ui/checkbox'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { ProductTypeMutateDialog } from '~/features/catalog/product-types/components/product-type-mutate-dialog'
import { WarehouseMutateDialog } from '~/features/catalog/warehouses/components/warehouse-mutate-dialog'
import { catalogStorageCreate, catalogStorageUpdate } from '~/generated/client'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { catalogStorageListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { queryClient } from '~/shared/api/query-client'

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
  const isUpdate = !!currentRow
  const idempotencyKey = useIdempotencyKey()

  const warehousesQuery = useCatalogWarehouseList()

  const productTypesQuery = useCatalogProductTypeList()

  const form = useForm<StorageFormValues>({
    resolver: zodResolver(storageFormSchema),
    defaultValues: {
      commonName: '',
      warehouseId: '',
      capacity: '',
      productTypeId: '',
      isTypeSpecific: false,
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        warehouseId: currentRow.warehouseId,
        capacity: currentRow.capacity ?? '',
        productTypeId: currentRow.productTypeId ?? '',
        isTypeSpecific: currentRow.isTypeSpecific,
      })
    }
    else {
      form.reset({
        commonName: '',
        warehouseId: '',
        capacity: '',
        productTypeId: '',
        isTypeSpecific: false,
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: StorageFormValues) => {
    try {
      const payload = {
        ...values,
        capacity: values.capacity || null,
        productTypeId: values.productTypeId || null,
      }

      if (isUpdate && currentRow) {
        await catalogStorageUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:storage.singular'),
          }),
        )
      }
      else {
        await catalogStorageCreate(payload, { headers: { 'Idempotency-Key': idempotencyKey } })
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:storage.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: catalogStorageListQueryKey() })
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(
        err instanceof Error ? err.message : t('common:toast.error'),
      )
    }
  }

  return (
    <FormDialog
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
      title={isUpdate ? t('catalog:storage.edit') : t('catalog:storage.create')}
      description={isUpdate ? t('catalog:storage.edit') : t('catalog:storage.create')}
      formId="storage-form"
    >
      <Form {...form}>
        <form
          id="storage-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="commonName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:storage.form.commonName')}</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <EntityPickerField<StorageFormValues>
            name="warehouseId"
            label={t('catalog:storage.form.warehouseId')}
            placeholder={t('catalog:storage.form.selectWarehouse')}
            queryResult={warehousesQuery}
            displayField="commonName"
            allowCreate
            createDialog={WarehouseMutateDialog}
          />
          <FormField
            control={form.control}
            name="capacity"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:storage.form.capacity')}</FormLabel>
                <FormControl>
                  <Input {...field} value={field.value ?? ''} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
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
          <FormField
            control={form.control}
            name="isTypeSpecific"
            render={({ field }) => (
              <FormItem className="flex items-center gap-2">
                <FormControl>
                  <Checkbox
                    checked={field.value}
                    onCheckedChange={field.onChange}
                  />
                </FormControl>
                <FormLabel className="!mt-0">
                  {t('catalog:storage.form.isTypeSpecific')}
                </FormLabel>
              </FormItem>
            )}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
