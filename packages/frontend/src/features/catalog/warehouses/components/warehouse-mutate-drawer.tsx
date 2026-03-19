import type { WarehouseResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { Button } from '~/components/ui/button'
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
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import {
  createWarehouse,
  invalidateWarehouses,
  updateWarehouse,
} from '../data/warehouse-api'

const warehouseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  baseId: z.string().min(1, 'Base is required'),
})

type WarehouseFormValues = z.infer<typeof warehouseFormSchema>

interface WarehouseMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: WarehouseResponse | null
}

export function WarehouseMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: WarehouseMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const { data: basesData } = useCatalogBaseList()
  const bases = basesData?.data ?? []

  const form = useForm<WarehouseFormValues>({
    resolver: zodResolver(warehouseFormSchema),
    defaultValues: {
      commonName: '',
      baseId: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        baseId: currentRow.baseId,
      })
    }
    else {
      form.reset({
        commonName: '',
        baseId: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: WarehouseFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await updateWarehouse(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:warehouse.singular'),
          }),
        )
      }
      else {
        await createWarehouse(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:warehouse.singular'),
          }),
        )
      }

      await invalidateWarehouses()
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
              ? t('catalog:warehouse.edit')
              : t('catalog:warehouse.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:warehouse.edit')
              : t('catalog:warehouse.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="warehouse-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:warehouse.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="baseId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:warehouse.form.baseId')}</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger className="w-full">
                        <SelectValue placeholder={t('catalog:warehouse.form.selectBase')} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {bases.map(b => (
                        <SelectItem key={b.id} value={b.id}>
                          {b.commonName}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="warehouse-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
