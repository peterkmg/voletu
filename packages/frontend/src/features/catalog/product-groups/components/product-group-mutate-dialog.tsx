import type { ProductGroupResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
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
import { ProductTypeMutateDialog } from '~/features/catalog/product-types/components/product-type-mutate-dialog'
import { catalogProductGroupCreate, catalogProductGroupUpdate } from '~/generated/client'
import { catalogProductGroupListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { queryClient } from '~/shared/api/query-client'

const productGroupFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  productTypeId: z.string().min(1, 'Product type is required'),
})

type ProductGroupFormValues = z.infer<typeof productGroupFormSchema>

interface ProductGroupMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductGroupResponse | null
  onCreated?: (id: string) => void
}

export function ProductGroupMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: ProductGroupMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow
  const idempotencyKey = useIdempotencyKey()

  const productTypesQuery = useCatalogProductTypeList()

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
        await catalogProductGroupUpdate(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:productGroup.singular'),
          }),
        )
      }
      else {
        const result = await catalogProductGroupCreate(values, { headers: { 'Idempotency-Key': idempotencyKey } })
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:productGroup.singular'),
          }),
        )
        if (onCreated && result?.data?.id) {
          onCreated(result.data.id)
        }
      }

      await queryClient.invalidateQueries({ queryKey: catalogProductGroupListQueryKey() })
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
      title={isUpdate ? t('catalog:productGroup.edit') : t('catalog:productGroup.create')}
      description={isUpdate ? t('catalog:productGroup.edit') : t('catalog:productGroup.create')}
      formId="product-group-form"
    >
      <Form {...form}>
        <form
          id="product-group-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
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
          <EntityPickerField<ProductGroupFormValues>
            name="productTypeId"
            label={t('catalog:productGroup.form.productType')}
            placeholder={t('catalog:productGroup.form.selectProductType')}
            queryResult={productTypesQuery}
            displayField="commonName"
            allowCreate
            createDialog={ProductTypeMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
