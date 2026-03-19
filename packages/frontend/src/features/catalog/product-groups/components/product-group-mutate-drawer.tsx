import type { ProductGroupResponse } from '~/generated/types'
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
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import {
  createProductGroup,
  invalidateProductGroups,
  updateProductGroup,
} from '../data/product-group-api'

const productGroupFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  productTypeId: z.string().min(1, 'Product type is required'),
})

type ProductGroupFormValues = z.infer<typeof productGroupFormSchema>

interface ProductGroupMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductGroupResponse | null
}

export function ProductGroupMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: ProductGroupMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const { data: productTypesData } = useCatalogProductTypeList()
  const productTypes = productTypesData?.data ?? []

  const form = useForm<ProductGroupFormValues>({
    resolver: zodResolver(productGroupFormSchema),
    defaultValues: {
      commonName: '',
      productTypeId: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        productTypeId: currentRow.productTypeId,
      })
    }
    else {
      form.reset({
        commonName: '',
        productTypeId: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: ProductGroupFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await updateProductGroup(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:productGroup.singular'),
          }),
        )
      }
      else {
        await createProductGroup(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:productGroup.singular'),
          }),
        )
      }

      await invalidateProductGroups()
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
              ? t('catalog:productGroup.edit')
              : t('catalog:productGroup.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:productGroup.edit')
              : t('catalog:productGroup.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="product-group-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:productGroup.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
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
                  <FormLabel>{t('catalog:productGroup.form.productType')}</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder={t('catalog:productGroup.form.selectProductType')} />
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
          </form>
        </Form>
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="product-group-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
