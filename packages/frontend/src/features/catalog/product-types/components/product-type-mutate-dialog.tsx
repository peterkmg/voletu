import type { ProductTypeResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
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
import { catalogProductTypeCreate, catalogProductTypeUpdate } from '~/generated/client'
import { catalogProductTypeListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { queryClient } from '~/shared/api/query-client'

const productTypeFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  longName: z.string().nullable().optional(),
})

type ProductTypeFormValues = z.infer<typeof productTypeFormSchema>

interface ProductTypeMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductTypeResponse | null
  onCreated?: (id: string) => void
}

export function ProductTypeMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: ProductTypeMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<ProductTypeFormValues>({
    resolver: zodResolver(productTypeFormSchema),
    defaultValues: {
      commonName: '',
      longName: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        longName: currentRow.longName ?? '',
      })
    }
    else {
      form.reset({
        commonName: '',
        longName: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: ProductTypeFormValues) => {
    try {
      const payload = {
        ...values,
        longName: values.longName || null,
      }

      if (isUpdate && currentRow) {
        await catalogProductTypeUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:productType.singular'),
          }),
        )
      }
      else {
        const result = await catalogProductTypeCreate(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:productType.singular'),
          }),
        )
        if (onCreated && result?.data?.id) {
          onCreated(result.data.id)
        }
      }

      await queryClient.invalidateQueries({ queryKey: catalogProductTypeListQueryKey() })
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
      title={isUpdate ? t('catalog:productType.edit') : t('catalog:productType.create')}
      description={isUpdate ? t('catalog:productType.edit') : t('catalog:productType.create')}
      formId="product-type-form"
    >
      <Form {...form}>
        <form
          id="product-type-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="commonName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:productType.form.commonName')}</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="longName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:productType.form.longName')}</FormLabel>
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
