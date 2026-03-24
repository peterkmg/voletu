import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { physicalTransferCreate } from '~/generated/client'
import { physicalTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const physicalTransferFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  startCargoOps: z.string().min(1, 'Start cargo ops is required'),
  endCargoOps: z.string().min(1, 'End cargo ops is required'),
})

type PhysicalTransferFormValues = z.infer<typeof physicalTransferFormSchema>

interface PhysicalTransferMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function PhysicalTransferMutateDialog({
  open,
  onOpenChange,
}: PhysicalTransferMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const { form, handleSubmit, handleOpenChange } = useMutateDialog<
    PhysicalTransferFormValues,
    { id: string },
    PhysicalTransferFormValues & { items: never[] }
  >({
    open,
    onOpenChange,
    schema: physicalTransferFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      startCargoOps: '',
      endCargoOps: '',
    },
    transformPayload: values => ({
      ...values,
      items: [],
    }),
    createFn: physicalTransferCreate,
    queryKey: physicalTransferListQueryKey(),
    entityLabel: t('documents:physicalTransfer.singular'),
    formId: 'physical-transfer-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={t('documents:physicalTransfer.create')}
      description={t('documents:physicalTransfer.create')}
      formId="physical-transfer-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="physical-transfer-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<PhysicalTransferFormValues> name="documentNumber" label={t('documents:acceptance.columns.documentNumber')} />
          <TextField<PhysicalTransferFormValues> name="date" label={t('documents:acceptance.columns.date')} type="datetime-local" />
          <TextField<PhysicalTransferFormValues> name="startCargoOps" label={t('common:table.startCargoOps', { defaultValue: 'Start Cargo Ops' })} type="datetime-local" />
          <TextField<PhysicalTransferFormValues> name="endCargoOps" label={t('common:table.endCargoOps', { defaultValue: 'End Cargo Ops' })} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}
