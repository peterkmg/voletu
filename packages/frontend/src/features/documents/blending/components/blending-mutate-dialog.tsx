import type { BlendingResponse } from '~/generated/types'
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
import { CompanyMutateDialog } from '~/features/catalog/companies/components/company-mutate-dialog'
import { blendingDocumentCreate, blendingDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { queryClient } from '~/shared/api/query-client'

const blendingFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  contractorId: z.string().min(1, 'Contractor is required'),
  targetProductId: z.string().min(1, 'Target product is required'),
})

type BlendingFormValues = z.infer<typeof blendingFormSchema>

interface BlendingMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BlendingResponse | null
}

export function BlendingMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: BlendingMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])
  const isUpdate = !!currentRow
  const idempotencyKey = useIdempotencyKey()

  const companiesQuery = useCatalogCompanyList()
  const productsQuery = useCatalogProductList()

  const form = useForm<BlendingFormValues>({
    resolver: zodResolver(blendingFormSchema),
    defaultValues: {
      documentNumber: '',
      date: '',
      contractorId: '',
      targetProductId: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        documentNumber: currentRow.documentNumber,
        date: currentRow.date ? currentRow.date.slice(0, 16) : '',
        contractorId: currentRow.contractorId,
        targetProductId: currentRow.targetProductId,
      })
    }
    else {
      form.reset({
        documentNumber: '',
        date: '',
        contractorId: '',
        targetProductId: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: BlendingFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await blendingDocumentUpdate(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('documents:blending.singular'),
          }),
        )
      }
      else {
        await blendingDocumentCreate(values, { headers: { 'Idempotency-Key': idempotencyKey } })
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('documents:blending.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: blendingDocumentListQueryKey() })
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
      title={isUpdate ? t('common:actions.edit') : t('documents:blending.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:blending.create')}
      formId="blending-form"
    >
      <Form {...form}>
        <form
          id="blending-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="documentNumber"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('documents:blending.columns.documentNumber')}</FormLabel>
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
                <FormLabel>{t('documents:blending.columns.date')}</FormLabel>
                <FormControl>
                  <Input type="datetime-local" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <EntityPickerField<BlendingFormValues>
            name="contractorId"
            label={t('documents:items.contractor')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
          <EntityPickerField<BlendingFormValues>
            name="targetProductId"
            label={t('documents:items.product')}
            queryResult={productsQuery}
            displayField="commonName"
          />
        </form>
      </Form>
    </FormDialog>
  )
}
