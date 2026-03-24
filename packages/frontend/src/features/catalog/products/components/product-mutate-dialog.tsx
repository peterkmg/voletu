import type { ProductResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { ProductGroupMutateDialog } from '~/features/catalog/product-groups/components/product-group-mutate-dialog'
import { catalogProductCreate, catalogProductUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { catalogProductListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

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

  const productGroupsQuery = useCatalogProductGroupList()
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, onSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: productFormSchema,
    defaultValues: {
      commonName: '',
      productGroupId: '',
      manufacturerId: '',
      addIdentification: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      productGroupId: row.productGroupId,
      manufacturerId: row.manufacturerId ?? '',
      addIdentification: row.addIdentification ?? '',
    }),
    transformPayload: values => ({
      ...values,
      manufacturerId: values.manufacturerId || null,
      addIdentification: values.addIdentification || null,
    }),
    createFn: catalogProductCreate,
    updateFn: catalogProductUpdate,
    queryKey: catalogProductListQueryKey(),
    entityLabel: t('catalog:product.singular'),
    formId: 'product-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:product.edit') : t('catalog:product.create')}
      description={isUpdate ? t('catalog:product.edit') : t('catalog:product.create')}
      formId="product-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="product-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <TextField<ProductFormValues> name="commonName" label={t('catalog:product.form.commonName')} />
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
          <TextField<ProductFormValues> name="addIdentification" label={t('catalog:product.form.identification')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}
