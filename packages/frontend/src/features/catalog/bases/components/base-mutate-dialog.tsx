import type { BaseResponse } from '~/generated/types/BaseResponse'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { FormDialog } from '~/components/form-dialog'
import { TextField } from '~/components/form-fields'
import { Form } from '~/components/ui/form'
import { catalogBaseCreate, catalogBaseUpdate } from '~/generated/client'
import { catalogBaseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'

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

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: baseFormSchema,
    defaultValues: {
      commonName: '',
      longName: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      longName: row.longName ?? '',
    }),
    transformPayload: values => ({
      ...values,
      longName: values.longName || null,
    }),
    createFn: catalogBaseCreate,
    updateFn: catalogBaseUpdate,
    queryKey: catalogBaseListQueryKey(),
    entityLabel: t('catalog:base.singular'),
    formId: 'base-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:base.edit') : t('catalog:base.create')}
      description={isUpdate ? t('catalog:base.edit') : t('catalog:base.create')}
      formId="base-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="base-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<BaseFormValues> name="commonName" label={t('catalog:base.form.commonName')} />
          <TextField<BaseFormValues> name="longName" label={t('catalog:base.form.longName')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}
