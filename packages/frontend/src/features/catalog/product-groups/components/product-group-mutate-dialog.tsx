import type { ProductGroupResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { ProductTypeMutateDialog } from '~/features/catalog/product-types/components/product-type-mutate-dialog'
import { catalogProductGroupCreate, catalogProductGroupUpdate } from '~/generated/client'
import { catalogProductGroupListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

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

  const productTypesQuery = useCatalogProductTypeList()

  const { form, isUpdate, onSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: productGroupFormSchema,
    defaultValues: {
      commonName: '',
      productTypeId: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      productTypeId: row.productTypeId,
    }),
    createFn: catalogProductGroupCreate,
    updateFn: catalogProductGroupUpdate,
    queryKey: catalogProductGroupListQueryKey(),
    entityLabel: t('catalog:productGroup.singular'),
    formId: 'product-group-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:productGroup.edit') : t('catalog:productGroup.create')}
      description={isUpdate ? t('catalog:productGroup.edit') : t('catalog:productGroup.create')}
      formId="product-group-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="product-group-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <TextField<ProductGroupFormValues> name="commonName" label={t('catalog:productGroup.form.commonName')} />
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
