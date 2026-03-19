import type { DispatchResponse } from '~/generated/types'
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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
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
  createDispatchDocument,
  invalidateDispatchDocuments,
  updateDispatchDocument,
} from '../data/dispatch-api'

const dispatchFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  dispatchPurpose: z.enum(['EXTERNAL', 'INTERNAL']),
  dispatchMethod: z.enum(['TRUCK', 'VESSEL_TERMINAL', 'BUNKERING']),
  contractorId: z.string().min(1, 'Contractor is required'),
  receiverEntity: z.string().nullable().optional(),
})

type DispatchFormValues = z.infer<typeof dispatchFormSchema>

interface DispatchMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: DispatchResponse | null
}

export function DispatchMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: DispatchMutateDrawerProps) {
  const { t } = useTranslation(['documents', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<DispatchFormValues>({
    resolver: zodResolver(dispatchFormSchema),
    defaultValues: {
      documentNumber: '',
      date: '',
      dispatchPurpose: 'EXTERNAL',
      dispatchMethod: 'TRUCK',
      contractorId: '',
      receiverEntity: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        documentNumber: currentRow.documentNumber,
        date: currentRow.date ? currentRow.date.slice(0, 16) : '',
        dispatchPurpose: currentRow.dispatchPurpose,
        dispatchMethod: currentRow.dispatchMethod,
        contractorId: currentRow.contractorId,
        receiverEntity: currentRow.receiverEntity ?? '',
      })
    }
    else {
      form.reset({
        documentNumber: '',
        date: '',
        dispatchPurpose: 'EXTERNAL',
        dispatchMethod: 'TRUCK',
        contractorId: '',
        receiverEntity: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: DispatchFormValues) => {
    try {
      const payload = {
        ...values,
        receiverEntity: values.receiverEntity || null,
      }

      if (isUpdate && currentRow) {
        await updateDispatchDocument(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('documents:dispatch.singular'),
          }),
        )
      }
      else {
        await createDispatchDocument(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('documents:dispatch.singular'),
          }),
        )
      }

      await invalidateDispatchDocuments()
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(
        err instanceof Error ? err.message : t('common:toast.error'),
      )
    }
  }

  const purposeOptions = ['EXTERNAL', 'INTERNAL'] as const
  const methodOptions = ['TRUCK', 'VESSEL_TERMINAL', 'BUNKERING'] as const

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
              ? t('common:actions.edit')
              : t('documents:dispatch.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('common:actions.edit')
              : t('documents:dispatch.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="dispatch-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="documentNumber"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:dispatch.columns.documentNumber')}</FormLabel>
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
                  <FormLabel>{t('documents:dispatch.columns.date')}</FormLabel>
                  <FormControl>
                    <Input type="datetime-local" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="dispatchPurpose"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:dispatch.columns.purpose')}</FormLabel>
                  <Select onValueChange={field.onChange} value={field.value}>
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {purposeOptions.map(option => (
                        <SelectItem key={option} value={option}>
                          {option}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="dispatchMethod"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:dispatch.columns.method')}</FormLabel>
                  <Select onValueChange={field.onChange} value={field.value}>
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {methodOptions.map(option => (
                        <SelectItem key={option} value={option}>
                          {option}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="contractorId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:items.contractor')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="receiverEntity"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Receiver Entity</FormLabel>
                  <FormControl>
                    <Input {...field} value={field.value ?? ''} />
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
          <Button form="dispatch-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
