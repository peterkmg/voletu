import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { ownershipTransferCreate } from '~/generated/client'
import { ownershipTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const ownershipTransferFormSchema = z.object({
  date: z.string().min(1, 'Date is required'),
})

type OwnershipTransferFormValues = z.infer<typeof ownershipTransferFormSchema>

interface OwnershipTransferMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function OwnershipTransferMutateDialog({
  open,
  onOpenChange,
}: OwnershipTransferMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const { form, handleSubmit, handleOpenChange } = useMutateDialog<
    OwnershipTransferFormValues,
    { id: string },
    OwnershipTransferFormValues & { items: never[] }
  >({
    open,
    onOpenChange,
    schema: ownershipTransferFormSchema,
    defaultValues: {
      date: '',
    },
    transformPayload: values => ({
      ...values,
      items: [],
    }),
    createFn: ownershipTransferCreate,
    queryKey: ownershipTransferListQueryKey(),
    entityLabel: t('documents:ownershipTransfer.singular'),
    formId: 'ownership-transfer-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={t('documents:ownershipTransfer.create')}
      description={t('documents:ownershipTransfer.create')}
      formId="ownership-transfer-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="ownership-transfer-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<OwnershipTransferFormValues> name="date" label={t('documents:acceptance.columns.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}
