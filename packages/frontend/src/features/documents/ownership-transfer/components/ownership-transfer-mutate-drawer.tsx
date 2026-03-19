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
  createOwnershipTransfer,
  invalidateOwnershipTransfers,
} from '../data/ownership-transfer-api'

const ownershipTransferFormSchema = z.object({
  date: z.string().min(1, 'Date is required'),
})

type OwnershipTransferFormValues = z.infer<typeof ownershipTransferFormSchema>

interface OwnershipTransferMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function OwnershipTransferMutateDrawer({
  open,
  onOpenChange,
}: OwnershipTransferMutateDrawerProps) {
  const { t } = useTranslation(['documents', 'common'])

  const form = useForm<OwnershipTransferFormValues>({
    resolver: zodResolver(ownershipTransferFormSchema),
    defaultValues: {
      date: '',
    },
  })

  const onSubmit = async (values: OwnershipTransferFormValues) => {
    try {
      await createOwnershipTransfer({
        ...values,
        items: [],
      })
      toast.success(
        t('common:toast.createSuccess', {
          entity: t('documents:ownershipTransfer.singular'),
        }),
      )
      await invalidateOwnershipTransfers()
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
            {t('documents:ownershipTransfer.create')}
          </SheetTitle>
          <SheetDescription>
            {t('documents:ownershipTransfer.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="ownership-transfer-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
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
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="ownership-transfer-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
