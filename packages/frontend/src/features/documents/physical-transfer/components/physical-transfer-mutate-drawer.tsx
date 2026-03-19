import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { Button } from '~/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from '~/components/ui/sheet'
import {
  createPhysicalTransfer,
  invalidatePhysicalTransfers,
} from '../data/physical-transfer-api'

const physicalTransferFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  startCargoOps: z.string().min(1, 'Start cargo ops is required'),
  endCargoOps: z.string().min(1, 'End cargo ops is required'),
})

type PhysicalTransferFormValues = z.infer<typeof physicalTransferFormSchema>

interface PhysicalTransferMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function PhysicalTransferMutateDrawer({
  open,
  onOpenChange,
}: PhysicalTransferMutateDrawerProps) {
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
      await createPhysicalTransfer({
        ...values,
        items: [],
      })
      toast.success(
        t('common:toast.createSuccess', {
          entity: t('documents:physicalTransfer.singular'),
        }),
      )
      await invalidatePhysicalTransfers()
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
    <Sheet
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
    >
      <SheetContent className="flex flex-col">
        <SheetHeader className="text-start">
          <SheetTitle>
            {t('documents:physicalTransfer.create')}
          </SheetTitle>
          <SheetDescription>
            {t('documents:physicalTransfer.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="physical-transfer-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
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
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="physical-transfer-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
