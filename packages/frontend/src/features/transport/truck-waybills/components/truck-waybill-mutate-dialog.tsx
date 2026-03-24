import type { TruckWaybillResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { transportTruckWaybillCreate, transportTruckWaybillUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { transportTruckWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const truckWaybillFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  senderId: z.string().min(1, 'Sender ID is required'),
})

type TruckWaybillFormValues = z.infer<typeof truckWaybillFormSchema>

interface TruckWaybillMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: TruckWaybillResponse | null
}

export function TruckWaybillMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: TruckWaybillMutateDialogProps) {
  const { t } = useTranslation(['transport', 'common'])

  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: truckWaybillFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      senderId: '',
    },
    mapRowToForm: (row: TruckWaybillResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date,
      senderId: row.senderId,
    }),
    createFn: transportTruckWaybillCreate,
    updateFn: transportTruckWaybillUpdate,
    queryKey: transportTruckWaybillListQueryKey(),
    entityLabel: t('transport:truck.singular'),
    formId: 'truck-waybill-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('transport:truck.edit') : t('transport:truck.create')}
      description={isUpdate ? t('transport:truck.edit') : t('transport:truck.create')}
      formId="truck-waybill-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="truck-waybill-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<TruckWaybillFormValues> name="documentNumber" label={t('transport:truck.form.documentNumber')} />
          <TextField<TruckWaybillFormValues> name="date" label={t('transport:truck.form.date')} type="date" />
          <EntityPickerField<TruckWaybillFormValues>
            name="senderId"
            label={t('transport:truck.form.senderId')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
