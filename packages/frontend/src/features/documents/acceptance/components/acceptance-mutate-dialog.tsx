import type { AcceptanceResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { SelectField, TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { acceptanceDocumentCreate, acceptanceDocumentUpdate } from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const arrivalTypes = ['TRUCK', 'RAIL', 'EXTERNAL', 'INITIAL_BALANCE'] as const

const acceptanceFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  dateAccepted: z.string().min(1, 'Date is required'),
  arrivalType: z.enum(arrivalTypes),
  sourceEntity: z.string().nullable().optional(),
})

type AcceptanceFormValues = z.infer<typeof acceptanceFormSchema>

interface AcceptanceMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: AcceptanceResponse | null
}

export function AcceptanceMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: AcceptanceMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const arrivalTypeOptions = arrivalTypes.map(type => ({
    value: type,
    label: t(`documents:acceptance.arrivalTypes.${type}`),
  }))

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog<
    AcceptanceFormValues,
    AcceptanceResponse,
    AcceptanceFormValues & { sourceEntity: string | null }
  >({
    open,
    onOpenChange,
    currentRow,
    schema: acceptanceFormSchema,
    defaultValues: {
      documentNumber: '',
      dateAccepted: '',
      arrivalType: 'TRUCK',
      sourceEntity: '',
    },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      dateAccepted: row.dateAccepted ? row.dateAccepted.slice(0, 16) : '',
      arrivalType: row.arrivalType,
      sourceEntity: row.sourceEntity ?? '',
    }),
    transformPayload: values => ({
      ...values,
      sourceEntity: values.sourceEntity || null,
    }),
    createFn: acceptanceDocumentCreate,
    updateFn: acceptanceDocumentUpdate,
    queryKey: acceptanceDocumentListQueryKey(),
    entityLabel: t('documents:acceptance.singular'),
    formId: 'acceptance-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('documents:acceptance.edit') : t('documents:acceptance.create')}
      description={isUpdate ? t('documents:acceptance.edit') : t('documents:acceptance.create')}
      formId="acceptance-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="acceptance-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<AcceptanceFormValues> name="documentNumber" label={t('documents:acceptance.form.documentNumber')} />
          <TextField<AcceptanceFormValues> name="dateAccepted" label={t('documents:acceptance.form.dateAccepted')} type="datetime-local" />
          <SelectField<AcceptanceFormValues> name="arrivalType" label={t('documents:acceptance.form.arrivalType')} options={arrivalTypeOptions} />
          <TextField<AcceptanceFormValues> name="sourceEntity" label={t('documents:acceptance.form.sourceEntity')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}
