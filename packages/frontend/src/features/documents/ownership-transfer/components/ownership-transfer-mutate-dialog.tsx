import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
import { toast } from 'sonner'
import { z } from 'zod'
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
import { ownershipTransferCreate } from '~/generated/client'
import { ownershipTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { queryClient } from '~/shared/api/query-client'

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
  const idempotencyKey = useIdempotencyKey()

  const form = useForm<OwnershipTransferFormValues>({
    resolver: zodResolver(ownershipTransferFormSchema),
    defaultValues: {
      date: '',
    },
  })

  const onSubmit = async (values: OwnershipTransferFormValues) => {
    try {
      await ownershipTransferCreate({
        ...values,
        items: [],
      }, { headers: { 'Idempotency-Key': idempotencyKey } })
      toast.success(
        t('common:toast.createSuccess', {
          entity: t('documents:ownershipTransfer.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: ownershipTransferListQueryKey() })
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
      title={t('documents:ownershipTransfer.create')}
      description={t('documents:ownershipTransfer.create')}
      formId="ownership-transfer-form"
    >
      <Form {...form}>
        <form
          id="ownership-transfer-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="date"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('documents:acceptance.columns.date')}</FormLabel>
                <FormControl>
                  <Input type="datetime-local" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
