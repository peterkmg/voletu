import type { InventoryReconciliationResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
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
import { WarehouseMutateDialog } from '~/features/catalog/warehouses/components/warehouse-mutate-dialog'
import { reconciliationCreate, reconciliationUpdate } from '~/generated/client'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { queryClient } from '~/shared/api/query-client'

const reconciliationFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  warehouseId: z.string().min(1, 'Warehouse is required'),
})

type ReconciliationFormValues = z.infer<typeof reconciliationFormSchema>

interface ReconciliationMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: InventoryReconciliationResponse | null
}

export function ReconciliationMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: ReconciliationMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])
  const isUpdate = !!currentRow

  const warehousesQuery = useCatalogWarehouseList()

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
    <FormDialog
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
      title={isUpdate ? t('common:actions.edit') : t('documents:reconciliation.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:reconciliation.create')}
      formId="reconciliation-form"
    >
      <Form {...form}>
        <form
          id="reconciliation-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
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
          <EntityPickerField<ReconciliationFormValues>
            name="warehouseId"
            label={t('common:nav.warehouses')}
            queryResult={warehousesQuery}
            displayField="commonName"
            allowCreate
            createDialog={WarehouseMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
