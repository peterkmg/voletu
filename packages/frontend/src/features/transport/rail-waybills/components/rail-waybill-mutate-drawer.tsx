import type { RailWaybillResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
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
  createRailWaybill,
  invalidateRailWaybills,
  updateRailWaybill,
} from '../data/rail-waybill-api'

const railWaybillFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  senderId: z.string().min(1, 'Sender ID is required'),
})

type RailWaybillFormValues = z.infer<typeof railWaybillFormSchema>

interface RailWaybillMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: RailWaybillResponse | null
}

export function RailWaybillMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: RailWaybillMutateDrawerProps) {
  const { t } = useTranslation(['transport', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<RailWaybillFormValues>({
    resolver: zodResolver(railWaybillFormSchema),
    defaultValues: {
      documentNumber: '',
      date: '',
      senderId: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        documentNumber: currentRow.documentNumber,
        date: currentRow.date,
        senderId: currentRow.senderId,
      })
    }
    else {
      form.reset({
        documentNumber: '',
        date: '',
        senderId: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: RailWaybillFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await updateRailWaybill(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('transport:rail.singular'),
          }),
        )
      }
      else {
        await createRailWaybill(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('transport:rail.singular'),
          }),
        )
      }

      await invalidateRailWaybills()
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
            {isUpdate
              ? t('transport:rail.edit')
              : t('transport:rail.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('transport:rail.edit')
              : t('transport:rail.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="rail-waybill-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="documentNumber"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('transport:rail.form.documentNumber')}</FormLabel>
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
                  <FormLabel>{t('transport:rail.form.date')}</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="senderId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('transport:rail.form.senderId')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
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
          <Button form="rail-waybill-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
