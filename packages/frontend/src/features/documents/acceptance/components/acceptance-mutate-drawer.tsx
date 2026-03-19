import type { AcceptanceResponse } from '~/generated/types'
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
import { acceptanceDocumentCreate, acceptanceDocumentUpdate } from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { queryClient } from '~/shared/api/query-client'

const arrivalTypes = ['TRUCK', 'RAIL', 'EXTERNAL', 'INITIAL_BALANCE'] as const

const acceptanceFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  dateAccepted: z.string().min(1, 'Date is required'),
  arrivalType: z.enum(arrivalTypes),
  sourceEntity: z.string().nullable().optional(),
})

type AcceptanceFormValues = z.infer<typeof acceptanceFormSchema>

interface AcceptanceMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: AcceptanceResponse | null
}

export function AcceptanceMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: AcceptanceMutateDrawerProps) {
  const { t } = useTranslation(['documents', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<AcceptanceFormValues>({
    resolver: zodResolver(acceptanceFormSchema),
    defaultValues: {
      documentNumber: '',
      dateAccepted: '',
      arrivalType: 'TRUCK',
      sourceEntity: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        documentNumber: currentRow.documentNumber,
        dateAccepted: currentRow.dateAccepted
          ? currentRow.dateAccepted.slice(0, 16)
          : '',
        arrivalType: currentRow.arrivalType,
        sourceEntity: currentRow.sourceEntity ?? '',
      })
    }
    else {
      form.reset({
        documentNumber: '',
        dateAccepted: '',
        arrivalType: 'TRUCK',
        sourceEntity: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: AcceptanceFormValues) => {
    try {
      const payload = {
        ...values,
        sourceEntity: values.sourceEntity || null,
      }

      if (isUpdate && currentRow) {
        await acceptanceDocumentUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('documents:acceptance.singular'),
          }),
        )
      }
      else {
        await acceptanceDocumentCreate(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('documents:acceptance.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: acceptanceDocumentListQueryKey() })
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
              ? t('documents:acceptance.edit')
              : t('documents:acceptance.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('documents:acceptance.edit')
              : t('documents:acceptance.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="acceptance-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="documentNumber"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:acceptance.form.documentNumber')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="dateAccepted"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:acceptance.form.dateAccepted')}</FormLabel>
                  <FormControl>
                    <Input type="datetime-local" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="arrivalType"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:acceptance.form.arrivalType')}</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {arrivalTypes.map(type => (
                        <SelectItem key={type} value={type}>
                          {t(`documents:acceptance.arrivalTypes.${type}`)}
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
              name="sourceEntity"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:acceptance.form.sourceEntity')}</FormLabel>
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
          <Button form="acceptance-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
