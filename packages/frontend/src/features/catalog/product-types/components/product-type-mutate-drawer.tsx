import type { ProductTypeResponse } from '~/generated/types'
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
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from '~/components/ui/sheet'
import {
  createProductType,
  invalidateProductTypes,
  updateProductType,
} from '../data/product-type-api'

const productTypeFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  longName: z.string().nullable().optional(),
})

type ProductTypeFormValues = z.infer<typeof productTypeFormSchema>

interface ProductTypeMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductTypeResponse | null
}

export function ProductTypeMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: ProductTypeMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<ProductTypeFormValues>({
    resolver: zodResolver(productTypeFormSchema),
    defaultValues: {
      commonName: '',
      longName: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        longName: currentRow.longName ?? '',
      })
    }
    else {
      form.reset({
        commonName: '',
        longName: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: ProductTypeFormValues) => {
    try {
      const payload = {
        ...values,
        longName: values.longName || null,
      }

      if (isUpdate && currentRow) {
        await updateProductType(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:productType.singular'),
          }),
        )
      }
      else {
        await createProductType(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:productType.singular'),
          }),
        )
      }

      await invalidateProductTypes()
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
              ? t('catalog:productType.edit')
              : t('catalog:productType.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:productType.edit')
              : t('catalog:productType.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="product-type-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:productType.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="longName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:productType.form.longName')}</FormLabel>
                  <FormControl>
                    <Input {...field} value={field.value ?? ''} />
                  </FormControl>
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
          <Button form="product-type-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
