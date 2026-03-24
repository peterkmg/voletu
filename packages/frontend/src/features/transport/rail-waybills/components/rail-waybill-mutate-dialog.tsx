import type { RailWaybillResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { transportRailWaybillCreate, transportRailWaybillUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { transportRailWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

const railWaybillFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  senderId: z.string().min(1, 'Sender ID is required'),
})

type RailWaybillFormValues = z.infer<typeof railWaybillFormSchema>

interface RailWaybillMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: RailWaybillResponse | null
}

export function RailWaybillMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: RailWaybillMutateDialogProps) {
  const { t } = useTranslation(['transport', 'common'])

  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, onSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: railWaybillFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      senderId: '',
    },
    mapRowToForm: (row: RailWaybillResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date,
      senderId: row.senderId,
    }),
    createFn: transportRailWaybillCreate,
    updateFn: transportRailWaybillUpdate,
    queryKey: transportRailWaybillListQueryKey(),
    entityLabel: t('transport:rail.singular'),
    formId: 'rail-waybill-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('transport:rail.edit') : t('transport:rail.create')}
      description={isUpdate ? t('transport:rail.edit') : t('transport:rail.create')}
      formId="rail-waybill-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="rail-waybill-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <TextField<RailWaybillFormValues> name="documentNumber" label={t('transport:rail.form.documentNumber')} />
          <TextField<RailWaybillFormValues> name="date" label={t('transport:rail.form.date')} type="date" />
          <EntityPickerField<RailWaybillFormValues>
            name="senderId"
            label={t('transport:rail.form.senderId')}
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
