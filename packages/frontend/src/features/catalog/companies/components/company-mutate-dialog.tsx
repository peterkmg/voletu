import type { CompanyResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { CheckboxField, TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { catalogCompanyCreate, catalogCompanyUpdate } from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const companyFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  legalName: z.string().nullable().optional(),
  isContractor: z.boolean(),
  isExporter: z.boolean(),
  isManufacturer: z.boolean(),
  isSender: z.boolean(),
})

type CompanyFormValues = z.infer<typeof companyFormSchema>

interface CompanyMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: CompanyResponse | null
  onCreated?: (id: string) => void
}

export function CompanyMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: CompanyMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const { form, isUpdate, onSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: companyFormSchema,
    defaultValues: {
      commonName: '',
      legalName: '',
      isContractor: false,
      isExporter: false,
      isManufacturer: false,
      isSender: false,
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      legalName: row.legalName ?? '',
      isContractor: row.isContractor,
      isExporter: row.isExporter,
      isManufacturer: row.isManufacturer,
      isSender: row.isSender,
    }),
    transformPayload: values => ({
      ...values,
      legalName: values.legalName || null,
    }),
    createFn: catalogCompanyCreate,
    updateFn: catalogCompanyUpdate,
    queryKey: catalogCompanyListQueryKey(),
    entityLabel: t('catalog:company.singular'),
    formId: 'company-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:company.edit') : t('catalog:company.create')}
      description={isUpdate ? t('catalog:company.edit') : t('catalog:company.create')}
      formId="company-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="company-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <TextField<CompanyFormValues> name="commonName" label={t('catalog:company.form.commonName')} />
          <TextField<CompanyFormValues> name="legalName" label={t('catalog:company.form.legalName')} nullable />
          <div className="space-y-3">
            <CheckboxField<CompanyFormValues> name="isContractor" label={t('catalog:company.form.isContractor')} />
            <CheckboxField<CompanyFormValues> name="isExporter" label={t('catalog:company.form.isExporter')} />
            <CheckboxField<CompanyFormValues> name="isManufacturer" label={t('catalog:company.form.isManufacturer')} />
            <CheckboxField<CompanyFormValues> name="isSender" label={t('catalog:company.form.isSender')} />
          </div>
        </form>
      </Form>
    </FormDialog>
  )
}
