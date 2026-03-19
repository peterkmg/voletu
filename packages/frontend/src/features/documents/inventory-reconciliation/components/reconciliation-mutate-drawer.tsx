import type { InventoryReconciliationResponse } from '~/generated/types'
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
import { reconciliationCreate, reconciliationUpdate } from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { queryClient } from '~/shared/api/query-client'

const reconciliationFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  warehouseId: z.string().min(1, 'Warehouse is required'),
})

type ReconciliationFormValues = z.infer<typeof reconciliationFormSchema>

interface ReconciliationMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: InventoryReconciliationResponse | null
}

export function ReconciliationMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: ReconciliationMutateDrawerProps) {
  const { t } = useTranslation(['documents', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<ReconciliationFormValues>({
    resolver: zodResolver(reconciliationFormSchema),
    defaultValues: {
      documentNumber: '',
      date: '',
      warehouseId: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        documentNumber: currentRow.documentNumber,
        date: currentRow.date ? currentRow.date.slice(0, 16) : '',
        warehouseId: currentRow.warehouseId,
      })
    }
    else {
      form.reset({
        documentNumber: '',
        date: '',
        warehouseId: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: ReconciliationFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await reconciliationUpdate(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('documents:reconciliation.singular'),
          }),
        )
      }
      else {
        await reconciliationCreate(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('documents:reconciliation.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: reconciliationListQueryKey() })
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
              ? t('common:actions.edit')
              : t('documents:reconciliation.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('common:actions.edit')
              : t('documents:reconciliation.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="reconciliation-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="documentNumber"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('documents:reconciliation.columns.documentNumber')}</FormLabel>
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
                  <FormLabel>{t('documents:reconciliation.columns.date')}</FormLabel>
                  <FormControl>
                    <Input type="datetime-local" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="warehouseId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('common:nav.warehouses')}</FormLabel>
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
          <Button form="reconciliation-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
