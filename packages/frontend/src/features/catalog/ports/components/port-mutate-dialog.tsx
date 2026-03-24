import type { PortResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { catalogPortCreate, catalogPortUpdate } from '~/generated/client'
import { catalogPortListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

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

  const { form, isUpdate, onSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: portFormSchema,
    defaultValues: {
      commonName: '',
      country: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      country: row.country ?? '',
    }),
    transformPayload: values => ({
      ...values,
      country: values.country || null,
    }),
    createFn: catalogPortCreate,
    updateFn: catalogPortUpdate,
    queryKey: catalogPortListQueryKey(),
    entityLabel: t('catalog:port.singular'),
    formId: 'port-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:port.edit') : t('catalog:port.create')}
      description={isUpdate ? t('catalog:port.edit') : t('catalog:port.create')}
      formId="port-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="port-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <TextField<PortFormValues> name="commonName" label={t('catalog:port.form.commonName')} />
          <TextField<PortFormValues> name="country" label={t('catalog:port.form.country')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}
