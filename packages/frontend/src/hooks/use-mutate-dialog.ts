import type { DefaultValues, FieldValues, UseFormReturn } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
import { extractErrorMessage } from '~/lib/error'
import { queryClient } from '~/shared/api/query-client'

interface UseMutateDialogConfig<
  TForm extends FieldValues,
  TRow extends { id: string },
  TPayload = TForm,
> {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: TRow | null
  schema: any
  defaultValues: DefaultValues<TForm>
  mapRowToForm?: (row: TRow) => DefaultValues<TForm>
  transformPayload?: (values: TForm) => TPayload
  createFn: (
    payload: TPayload,
    opts?: { headers?: Record<string, string> },
  ) => Promise<{ data?: { id?: string } } | unknown>
  updateFn?: (id: string, payload: TPayload) => Promise<unknown>
  queryKey: readonly unknown[]
  entityLabel: string
  idempotency?: boolean
  formId: string
  onCreated?: (id: string) => void
}

interface UseMutateDialogReturn<TForm extends FieldValues> {
  form: UseFormReturn<TForm>
  isUpdate: boolean
  onSubmit: (values: TForm) => Promise<void>
  handleOpenChange: (open: boolean) => void
}

export function useMutateDialog<
  TForm extends FieldValues,
  TRow extends { id: string },
  TPayload = TForm,
>({
  onOpenChange,
  currentRow,
  schema,
  defaultValues,
  mapRowToForm,
  transformPayload,
  createFn,
  updateFn,
  queryKey,
  entityLabel,
  idempotency = true,
  onCreated,
}: UseMutateDialogConfig<TForm, TRow, TPayload>): UseMutateDialogReturn<TForm> {
  const { t } = useTranslation('common')
  const isUpdate = !!currentRow && !!updateFn
  const idempotencyKey = useIdempotencyKey()

  const form = useForm<TForm>({
    resolver: zodResolver(schema),
    defaultValues,
  })

  useEffect(() => {
    if (currentRow && mapRowToForm) {
      form.reset(mapRowToForm(currentRow))
    }
    else {
      form.reset(defaultValues)
    }
  }, [currentRow, form, defaultValues, mapRowToForm])

  const onSubmit = async (values: TForm) => {
    try {
      const payload = (
        transformPayload ? transformPayload(values) : values
      ) as TPayload

      if (isUpdate && currentRow) {
        await updateFn!(currentRow.id, payload)
        toast.success(t('toast.updateSuccess', { entity: entityLabel }))
      }
      else {
        const headers = idempotency
          ? { 'Idempotency-Key': idempotencyKey }
          : undefined
        const result = await createFn(
          payload,
          headers ? { headers } : undefined,
        )
        toast.success(t('toast.createSuccess', { entity: entityLabel }))

        if (onCreated) {
          const id = (result as { data?: { id?: string } })?.data?.id
          if (id)
            onCreated(id)
        }
      }

      await queryClient.invalidateQueries({ queryKey })
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(extractErrorMessage(err, t('toast.error')))
    }
  }

  const handleOpenChange = (v: boolean) => {
    onOpenChange(v)
    form.reset()
  }

  return { form, isUpdate, onSubmit, handleOpenChange }
}
