import type { CompanyResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { Checkbox } from '~/components/ui/checkbox'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { catalogCompanyCreate, catalogCompanyUpdate } from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { queryClient } from '~/shared/api/query-client'

const companyFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  legalName: z.string().nullable().optional(),
  isContractor: z.boolean(),
  isExporter: z.boolean(),
  isManufacturer: z.boolean(),
  isSender: z.boolean(),
})

type CompanyFormValues = z.infer<typeof companyFormSchema>

interface CompanyMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: CompanyResponse | null
  onCreated?: (id: string) => void
}

export function CompanyMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: CompanyMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<CompanyFormValues>({
    resolver: zodResolver(companyFormSchema),
    defaultValues: {
      commonName: '',
      legalName: '',
      isContractor: false,
      isExporter: false,
      isManufacturer: false,
      isSender: false,
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        legalName: currentRow.legalName ?? '',
        isContractor: currentRow.isContractor,
        isExporter: currentRow.isExporter,
        isManufacturer: currentRow.isManufacturer,
        isSender: currentRow.isSender,
      })
    }
    else {
      form.reset({
        commonName: '',
        legalName: '',
        isContractor: false,
        isExporter: false,
        isManufacturer: false,
        isSender: false,
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: CompanyFormValues) => {
    try {
      const payload = {
        ...values,
        legalName: values.legalName || null,
      }

      if (isUpdate && currentRow) {
        await catalogCompanyUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:company.singular'),
          }),
        )
      }
      else {
        const result = await catalogCompanyCreate(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:company.singular'),
          }),
        )
        if (onCreated && result?.data?.id) {
          onCreated(result.data.id)
        }
      }

      await queryClient.invalidateQueries({ queryKey: catalogCompanyListQueryKey() })
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(
        err instanceof Error ? err.message : t('common:toast.error'),
      )
    }
  }

  const booleanFields = [
    { name: 'isContractor' as const, label: t('catalog:company.form.isContractor') },
    { name: 'isExporter' as const, label: t('catalog:company.form.isExporter') },
    { name: 'isManufacturer' as const, label: t('catalog:company.form.isManufacturer') },
    { name: 'isSender' as const, label: t('catalog:company.form.isSender') },
  ]

  return (
    <FormDialog
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
      title={isUpdate ? t('catalog:company.edit') : t('catalog:company.create')}
      description={isUpdate ? t('catalog:company.edit') : t('catalog:company.create')}
      formId="company-form"
    >
      <Form {...form}>
        <form
          id="company-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="commonName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:company.form.commonName')}</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="legalName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:company.form.legalName')}</FormLabel>
                <FormControl>
                  <Input {...field} value={field.value ?? ''} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <div className="space-y-3">
            {booleanFields.map(({ name, label }) => (
              <FormField
                key={name}
                control={form.control}
                name={name}
                render={({ field }) => (
                  <FormItem className="flex items-center gap-2">
                    <FormControl>
                      <Checkbox
                        checked={field.value}
                        onCheckedChange={field.onChange}
                      />
                    </FormControl>
                    <FormLabel className="!mt-0">{label}</FormLabel>
                  </FormItem>
                )}
              />
            ))}
          </div>
        </form>
      </Form>
    </FormDialog>
  )
}
