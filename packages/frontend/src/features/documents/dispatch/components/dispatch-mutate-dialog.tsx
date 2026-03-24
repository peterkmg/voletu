import type { DispatchResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { SelectField, TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { dispatchDocumentCreate, dispatchDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const dispatchFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  dispatchPurpose: z.enum(['EXTERNAL', 'INTERNAL']),
  dispatchMethod: z.enum(['TRUCK', 'VESSEL_TERMINAL', 'BUNKERING']),
  contractorId: z.string().min(1, 'Contractor is required'),
  receiverEntity: z.string().nullable().optional(),
})

type DispatchFormValues = z.infer<typeof dispatchFormSchema>

const purposeOptions = [
  { value: 'EXTERNAL', label: 'EXTERNAL' },
  { value: 'INTERNAL', label: 'INTERNAL' },
] as const

const methodOptions = [
  { value: 'TRUCK', label: 'TRUCK' },
  { value: 'VESSEL_TERMINAL', label: 'VESSEL_TERMINAL' },
  { value: 'BUNKERING', label: 'BUNKERING' },
] as const

interface DispatchMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: DispatchResponse | null
}

export function DispatchMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: DispatchMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog<
    DispatchFormValues,
    DispatchResponse,
    DispatchFormValues & { receiverEntity: string | null }
  >({
    open,
    onOpenChange,
    currentRow,
    schema: dispatchFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      dispatchPurpose: 'EXTERNAL',
      dispatchMethod: 'TRUCK',
      contractorId: '',
      receiverEntity: '',
    },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      date: row.date ? row.date.slice(0, 16) : '',
      dispatchPurpose: row.dispatchPurpose,
      dispatchMethod: row.dispatchMethod,
      contractorId: row.contractorId,
      receiverEntity: row.receiverEntity ?? '',
    }),
    transformPayload: values => ({
      ...values,
      receiverEntity: values.receiverEntity || null,
    }),
    createFn: dispatchDocumentCreate,
    updateFn: dispatchDocumentUpdate,
    queryKey: dispatchDocumentListQueryKey(),
    entityLabel: t('documents:dispatch.singular'),
    formId: 'dispatch-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('documents:dispatch.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:dispatch.create')}
      formId="dispatch-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="dispatch-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<DispatchFormValues> name="documentNumber" label={t('documents:dispatch.columns.documentNumber')} />
          <TextField<DispatchFormValues> name="date" label={t('documents:dispatch.columns.date')} type="datetime-local" />
          <SelectField<DispatchFormValues> name="dispatchPurpose" label={t('documents:dispatch.columns.purpose')} options={purposeOptions} />
          <SelectField<DispatchFormValues> name="dispatchMethod" label={t('documents:dispatch.columns.method')} options={methodOptions} />
          <EntityPickerField<DispatchFormValues>
            name="contractorId"
            label={t('documents:items.contractor')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
          <TextField<DispatchFormValues> name="receiverEntity" label="Receiver Entity" nullable />
        </form>
      </Form>
    </FormDialog>
  )
}
