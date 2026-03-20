import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
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
import { physicalTransferCreate } from '~/generated/client'
import { physicalTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { queryClient } from '~/shared/api/query-client'

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

  const form = useForm<PhysicalTransferFormValues>({
    resolver: zodResolver(physicalTransferFormSchema),
    defaultValues: {
      documentNumber: '',
      date: '',
      startCargoOps: '',
      endCargoOps: '',
    },
  })

  const onSubmit = async (values: PhysicalTransferFormValues) => {
    try {
      await physicalTransferCreate({
        ...values,
        items: [],
      })
      toast.success(
        t('common:toast.createSuccess', {
          entity: t('documents:physicalTransfer.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: physicalTransferListQueryKey() })
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
      title={t('documents:physicalTransfer.create')}
      description={t('documents:physicalTransfer.create')}
      formId="physical-transfer-form"
    >
      <Form {...form}>
        <form
          id="physical-transfer-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="documentNumber"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('documents:acceptance.columns.documentNumber')}</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
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
          <FormField
            control={form.control}
            name="startCargoOps"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('common:table.startCargoOps', { defaultValue: 'Start Cargo Ops' })}</FormLabel>
                <FormControl>
                  <Input type="datetime-local" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="endCargoOps"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('common:table.endCargoOps', { defaultValue: 'End Cargo Ops' })}</FormLabel>
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
