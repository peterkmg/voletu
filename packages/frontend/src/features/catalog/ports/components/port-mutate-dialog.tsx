import type { PortResponse } from '~/generated/types'
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
import { catalogPortCreate, catalogPortUpdate } from '~/generated/client'
import { catalogPortListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { queryClient } from '~/shared/api/query-client'

const portFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  country: z.string().nullable().optional(),
})

type PortFormValues = z.infer<typeof portFormSchema>

interface PortMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: PortResponse | null
}

export function PortMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: PortMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow
  const idempotencyKey = useIdempotencyKey()

  const form = useForm<PortFormValues>({
    resolver: zodResolver(portFormSchema),
    defaultValues: {
      commonName: '',
      country: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        country: currentRow.country ?? '',
      })
    }
    else {
      form.reset({
        commonName: '',
        country: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: PortFormValues) => {
    try {
      const payload = {
        ...values,
        country: values.country || null,
      }

      if (isUpdate && currentRow) {
        await catalogPortUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:port.singular'),
          }),
        )
      }
      else {
        await catalogPortCreate(payload, { headers: { 'Idempotency-Key': idempotencyKey } })
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:port.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: catalogPortListQueryKey() })
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
      title={isUpdate ? t('catalog:port.edit') : t('catalog:port.create')}
      description={isUpdate ? t('catalog:port.edit') : t('catalog:port.create')}
      formId="port-form"
    >
      <Form {...form}>
        <form
          id="port-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="commonName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:port.form.commonName')}</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="country"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:port.form.country')}</FormLabel>
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
