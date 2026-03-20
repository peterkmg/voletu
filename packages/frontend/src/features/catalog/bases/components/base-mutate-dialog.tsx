import type { BaseResponse } from '~/generated/types/BaseResponse'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
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
import { catalogBaseCreate, catalogBaseUpdate } from '~/generated/client'
import { catalogBaseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { queryClient } from '~/shared/api/query-client'

const baseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  longName: z.string().nullable().optional(),
})

type BaseFormValues = z.infer<typeof baseFormSchema>

interface BaseMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BaseResponse | null
  onCreated?: (id: string) => void
}

export function BaseMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: BaseMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow
  const idempotencyKey = useIdempotencyKey()

  const form = useForm<BaseFormValues>({
    resolver: zodResolver(baseFormSchema),
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

  const onSubmit = async (values: BaseFormValues) => {
    try {
      const payload = {
        ...values,
        longName: values.longName || null,
      }

      if (isUpdate && currentRow) {
        await catalogBaseUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:base.singular'),
          }),
        )
      }
      else {
        const result = await catalogBaseCreate(payload, { headers: { 'Idempotency-Key': idempotencyKey } })
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:base.singular'),
          }),
        )
        if (onCreated && result?.data?.id) {
          onCreated(result.data.id)
        }
      }

      await queryClient.invalidateQueries({ queryKey: catalogBaseListQueryKey() })
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
      title={isUpdate ? t('catalog:base.edit') : t('catalog:base.create')}
      description={isUpdate ? t('catalog:base.edit') : t('catalog:base.create')}
      formId="base-form"
    >
      <Form {...form}>
        <form
          id="base-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="commonName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:base.form.commonName')}</FormLabel>
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
                <FormLabel>{t('catalog:base.form.longName')}</FormLabel>
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
