import type { WarehouseResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { BaseMutateDialog } from '~/features/catalog/bases/components/base-mutate-dialog'
import { catalogWarehouseCreate, catalogWarehouseUpdate } from '~/generated/client'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { catalogWarehouseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const warehouseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  baseId: z.string().min(1, 'Base is required'),
})

type WarehouseFormValues = z.infer<typeof warehouseFormSchema>

interface WarehouseMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: WarehouseResponse | null
  onCreated?: (id: string) => void
}

export function WarehouseMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: WarehouseMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const basesQuery = useCatalogBaseList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: warehouseFormSchema,
    defaultValues: {
      commonName: '',
      baseId: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      baseId: row.baseId,
    }),
    createFn: catalogWarehouseCreate,
    updateFn: catalogWarehouseUpdate,
    queryKey: catalogWarehouseListQueryKey(),
    entityLabel: t('catalog:warehouse.singular'),
    formId: 'warehouse-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      description={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      formId="warehouse-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="warehouse-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<WarehouseFormValues> name="commonName" label={t('catalog:warehouse.form.commonName')} />
          <EntityPickerField<WarehouseFormValues>
            name="baseId"
            label={t('catalog:warehouse.form.baseId')}
            placeholder={t('catalog:warehouse.form.selectBase')}
            queryResult={basesQuery}
            displayField="commonName"
            allowCreate
            createDialog={BaseMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
