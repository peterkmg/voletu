import type { StorageResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { Button } from '~/components/ui/button'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from '~/components/ui/sheet'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { catalogStorageCreate, catalogStorageUpdate } from '~/generated/client'
import { catalogStorageListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { queryClient } from '~/shared/api/query-client'

const storageFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  warehouseId: z.string().min(1, 'Warehouse is required'),
  capacity: z.string().nullable().optional(),
  productTypeId: z.string().nullable().optional(),
  isTypeSpecific: z.boolean(),
})

type StorageFormValues = z.infer<typeof storageFormSchema>

interface StorageMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: StorageResponse | null
}

export function StorageMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: StorageMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const { data: warehousesData } = useCatalogWarehouseList()
  const warehouses = warehousesData?.data ?? []

  const { data: productTypesData } = useCatalogProductTypeList()
  const productTypes = productTypesData?.data ?? []

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
        await catalogStorageCreate(payload)
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
    <Sheet
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
    >
      <SheetContent className="flex flex-col">
        <SheetHeader className="text-start">
          <SheetTitle>
            {isUpdate
              ? t('catalog:storage.edit')
              : t('catalog:storage.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:storage.edit')
              : t('catalog:storage.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="storage-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
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
            <FormField
              control={form.control}
              name="warehouseId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:storage.form.warehouseId')}</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger className="w-full">
                        <SelectValue placeholder={t('catalog:storage.form.selectWarehouse')} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {warehouses.map(wh => (
                        <SelectItem key={wh.id} value={wh.id}>
                          {wh.commonName}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
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
            <FormField
              control={form.control}
              name="productTypeId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:storage.form.productTypeId')}</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value ?? ''}
                  >
                    <FormControl>
                      <SelectTrigger className="w-full">
                        <SelectValue placeholder={t('catalog:storage.form.selectProductType')} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {productTypes.map(pt => (
                        <SelectItem key={pt.id} value={pt.id}>
                          {pt.commonName}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
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
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="storage-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
