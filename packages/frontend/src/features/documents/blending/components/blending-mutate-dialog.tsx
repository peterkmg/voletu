import type { BlendingResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { blendingDocumentCreate, blendingDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const blendingFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  contractorId: z.string().min(1, 'Contractor is required'),
  targetProductId: z.string().min(1, 'Target product is required'),
})

type BlendingFormValues = z.infer<typeof blendingFormSchema>

interface BlendingMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BlendingResponse | null
}

export function BlendingMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: BlendingMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const companiesQuery = useCatalogCompanyList()
  const productsQuery = useCatalogProductList()

  const { form, isUpdate, onSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: blendingFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      contractorId: '',
      targetProductId: '',
    },
    mapRowToForm: (row: BlendingResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date ? row.date.slice(0, 16) : '',
      contractorId: row.contractorId,
      targetProductId: row.targetProductId,
    }),
    createFn: blendingDocumentCreate,
    updateFn: blendingDocumentUpdate,
    queryKey: blendingDocumentListQueryKey(),
    entityLabel: t('documents:blending.singular'),
    formId: 'blending-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('documents:blending.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:blending.create')}
      formId="blending-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="blending-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <TextField<BlendingFormValues> name="documentNumber" label={t('documents:blending.columns.documentNumber')} />
          <TextField<BlendingFormValues> name="date" label={t('documents:blending.columns.date')} type="datetime-local" />
          <EntityPickerField<BlendingFormValues>
            name="contractorId"
            label={t('documents:items.contractor')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
          <EntityPickerField<BlendingFormValues>
            name="targetProductId"
            label={t('documents:items.product')}
            queryResult={productsQuery}
            displayField="commonName"
          />
        </form>
      </Form>
    </FormDialog>
  )
}
