import type { ProductResponse } from '~/generated/types'
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
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import {
  createProduct,
  invalidateProducts,
  updateProduct,
} from '../data/product-api'

const productFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  productGroupId: z.string().min(1, 'Product group is required'),
  manufacturerId: z.string().nullable().optional(),
  addIdentification: z.string().nullable().optional(),
})

type ProductFormValues = z.infer<typeof productFormSchema>

interface ProductMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductResponse | null
}

export function ProductMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: ProductMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const { data: productGroupsData } = useCatalogProductGroupList()
  const productGroups = productGroupsData?.data ?? []

  const { data: companiesData } = useCatalogCompanyList()
  const companies = companiesData?.data ?? []

  const form = useForm<ProductFormValues>({
    resolver: zodResolver(productFormSchema),
    defaultValues: {
      commonName: '',
      productGroupId: '',
      manufacturerId: '',
      addIdentification: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        productGroupId: currentRow.productGroupId,
        manufacturerId: currentRow.manufacturerId ?? '',
        addIdentification: currentRow.addIdentification ?? '',
      })
    }
    else {
      form.reset({
        commonName: '',
        productGroupId: '',
        manufacturerId: '',
        addIdentification: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: ProductFormValues) => {
    try {
      const payload = {
        ...values,
        manufacturerId: values.manufacturerId || null,
        addIdentification: values.addIdentification || null,
      }

      if (isUpdate && currentRow) {
        await updateProduct(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:product.singular'),
          }),
        )
      }
      else {
        await createProduct(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:product.singular'),
          }),
        )
      }

      await invalidateProducts()
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
              ? t('catalog:product.edit')
              : t('catalog:product.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:product.edit')
              : t('catalog:product.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="product-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:product.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="productGroupId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:product.form.productGroupId')}</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger className="w-full">
                        <SelectValue placeholder={t('catalog:product.form.selectProductGroup')} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {productGroups.map(pg => (
                        <SelectItem key={pg.id} value={pg.id}>
                          {pg.commonName}
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
              name="manufacturerId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:product.form.manufacturerId')}</FormLabel>
                  <Select
                    onValueChange={val => field.onChange(val === '__none__' ? '' : val)}
                    value={field.value ?? ''}
                  >
                    <FormControl>
                      <SelectTrigger className="w-full">
                        <SelectValue placeholder={t('catalog:product.form.selectManufacturer')} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value="__none__">—</SelectItem>
                      {companies.map(c => (
                        <SelectItem key={c.id} value={c.id}>
                          {c.commonName}
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
              name="addIdentification"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:product.form.identification')}</FormLabel>
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
          <Button form="product-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
