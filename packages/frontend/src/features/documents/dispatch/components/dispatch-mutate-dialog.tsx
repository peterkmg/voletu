import type { DispatchResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { dispatchDocumentCreate, dispatchDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { queryClient } from '~/shared/api/query-client'

const dispatchFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  dispatchPurpose: z.enum(['EXTERNAL', 'INTERNAL']),
  dispatchMethod: z.enum(['TRUCK', 'VESSEL_TERMINAL', 'BUNKERING']),
  contractorId: z.string().min(1, 'Contractor is required'),
  receiverEntity: z.string().nullable().optional(),
})

type DispatchFormValues = z.infer<typeof dispatchFormSchema>

interface DispatchMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: DispatchResponse | null
}

export function DispatchMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: DispatchMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])
  const isUpdate = !!currentRow
  const idempotencyKey = useIdempotencyKey()

  const companiesQuery = useCatalogCompanyList()

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
        await dispatchDocumentUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('documents:dispatch.singular'),
          }),
        )
      }
      else {
        await dispatchDocumentCreate(payload, { headers: { 'Idempotency-Key': idempotencyKey } })
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('documents:dispatch.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: dispatchDocumentListQueryKey() })
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
    <FormDialog
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
      title={isUpdate ? t('common:actions.edit') : t('documents:dispatch.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:dispatch.create')}
      formId="dispatch-form"
    >
      <Form {...form}>
        <form
          id="dispatch-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
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
          <EntityPickerField<DispatchFormValues>
            name="contractorId"
            label={t('documents:items.contractor')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
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
    </FormDialog>
  )
}
