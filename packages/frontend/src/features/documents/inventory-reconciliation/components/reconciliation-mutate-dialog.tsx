import type { InventoryReconciliationResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { WarehouseMutateDialog } from '~/features/catalog/warehouses/components/warehouse-mutate-dialog'
import { reconciliationCreate, reconciliationUpdate } from '~/generated/client'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

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

  const warehousesQuery = useCatalogWarehouseList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: reconciliationFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      warehouseId: '',
    },
    mapRowToForm: (row: InventoryReconciliationResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date ? row.date.slice(0, 16) : '',
      warehouseId: row.warehouseId,
    }),
    createFn: reconciliationCreate,
    updateFn: reconciliationUpdate,
    queryKey: reconciliationListQueryKey(),
    entityLabel: t('documents:reconciliation.singular'),
    formId: 'reconciliation-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('documents:reconciliation.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:reconciliation.create')}
      formId="reconciliation-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="reconciliation-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<ReconciliationFormValues> name="documentNumber" label={t('documents:reconciliation.columns.documentNumber')} />
          <TextField<ReconciliationFormValues> name="date" label={t('documents:reconciliation.columns.date')} type="datetime-local" />
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
