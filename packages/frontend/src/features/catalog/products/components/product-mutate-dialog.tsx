import type { ProductResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { ProductGroupMutateDialog } from '~/features/catalog/product-groups/components/product-group-mutate-dialog'
import { catalogProductCreate, catalogProductUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { catalogProductListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { queryClient } from '~/shared/api/query-client'

const productFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  productGroupId: z.string().min(1, 'Product group is required'),
  manufacturerId: z.string().nullable().optional(),
  addIdentification: z.string().nullable().optional(),
})

type ProductFormValues = z.infer<typeof productFormSchema>

interface ProductMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductResponse | null
}

export function ProductMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: ProductMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const productGroupsQuery = useCatalogProductGroupList()

  const companiesQuery = useCatalogCompanyList()

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
        await catalogProductUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:product.singular'),
          }),
        )
      }
      else {
        await catalogProductCreate(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:product.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: catalogProductListQueryKey() })
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
      title={isUpdate ? t('catalog:product.edit') : t('catalog:product.create')}
      description={isUpdate ? t('catalog:product.edit') : t('catalog:product.create')}
      formId="product-form"
    >
      <Form {...form}>
        <form
          id="product-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
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
          <EntityPickerField<ProductFormValues>
            name="productGroupId"
            label={t('catalog:product.form.productGroupId')}
            placeholder={t('catalog:product.form.selectProductGroup')}
            queryResult={productGroupsQuery}
            displayField="commonName"
            allowCreate
            createDialog={ProductGroupMutateDialog}
          />
          <EntityPickerField<ProductFormValues>
            name="manufacturerId"
            label={t('catalog:product.form.manufacturerId')}
            placeholder={t('catalog:product.form.selectManufacturer')}
            queryResult={companiesQuery}
            displayField="commonName"
            nullable
            allowCreate
            createDialog={CompanyMutateDialog}
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
    </FormDialog>
  )
}
