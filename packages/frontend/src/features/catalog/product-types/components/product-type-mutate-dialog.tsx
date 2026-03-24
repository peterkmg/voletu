import type { ProductTypeResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { catalogProductTypeCreate, catalogProductTypeUpdate } from '~/generated/client'
import { catalogProductTypeListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const productTypeFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  longName: z.string().nullable().optional(),
})

type ProductTypeFormValues = z.infer<typeof productTypeFormSchema>

interface ProductTypeMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductTypeResponse | null
  onCreated?: (id: string) => void
}

export function ProductTypeMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: ProductTypeMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: productTypeFormSchema,
    defaultValues: {
      commonName: '',
      longName: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      longName: row.longName ?? '',
    }),
    transformPayload: values => ({
      ...values,
      longName: values.longName || null,
    }),
    createFn: catalogProductTypeCreate,
    updateFn: catalogProductTypeUpdate,
    queryKey: catalogProductTypeListQueryKey(),
    entityLabel: t('catalog:productType.singular'),
    formId: 'product-type-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:productType.edit') : t('catalog:productType.create')}
      description={isUpdate ? t('catalog:productType.edit') : t('catalog:productType.create')}
      formId="product-type-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="product-type-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<ProductTypeFormValues> name="commonName" label={t('catalog:productType.form.commonName')} />
          <TextField<ProductTypeFormValues> name="longName" label={t('catalog:productType.form.longName')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}
