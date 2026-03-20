import type { RailWaybillResponse } from '~/generated/types'
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
import { transportRailWaybillCreate, transportRailWaybillUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { transportRailWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { queryClient } from '~/shared/api/query-client'

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
  const isUpdate = !!currentRow

  const companiesQuery = useCatalogCompanyList()

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
        await transportRailWaybillUpdate(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('transport:rail.singular'),
          }),
        )
      }
      else {
        await transportRailWaybillCreate(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('transport:rail.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: transportRailWaybillListQueryKey() })
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
      title={isUpdate ? t('transport:rail.edit') : t('transport:rail.create')}
      description={isUpdate ? t('transport:rail.edit') : t('transport:rail.create')}
      formId="rail-waybill-form"
    >
      <Form {...form}>
        <form
          id="rail-waybill-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
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
