import type { TruckWaybillResponse } from '~/generated/types'
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
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { transportTruckWaybillCreate, transportTruckWaybillUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { transportTruckWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { queryClient } from '~/shared/api/query-client'

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
  const isUpdate = !!currentRow

  const companiesQuery = useCatalogCompanyList()

  const form = useForm<TruckWaybillFormValues>({
    resolver: zodResolver(truckWaybillFormSchema),
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

  const onSubmit = async (values: TruckWaybillFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await transportTruckWaybillUpdate(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('transport:truck.singular'),
          }),
        )
      }
      else {
        await transportTruckWaybillCreate(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('transport:truck.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: transportTruckWaybillListQueryKey() })
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
      title={isUpdate ? t('transport:truck.edit') : t('transport:truck.create')}
      description={isUpdate ? t('transport:truck.edit') : t('transport:truck.create')}
      formId="truck-waybill-form"
    >
      <Form {...form}>
        <form
          id="truck-waybill-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="documentNumber"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('transport:truck.form.documentNumber')}</FormLabel>
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
                <FormLabel>{t('transport:truck.form.date')}</FormLabel>
                <FormControl>
                  <Input type="date" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
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
