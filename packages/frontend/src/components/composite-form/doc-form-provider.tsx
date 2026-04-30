import type { MutableRefObject, ReactNode } from 'react'
import type { DefaultValues, FieldValues, UseFormReturn } from 'react-hook-form'
import type {
  CompositeMutationFn,
  CompositeSuccessHandler,
  ServerValidationIssue,
} from './types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect, useState } from 'react'
import {

  FormProvider,
  useForm,
} from 'react-hook-form'
import { applyServerErrors } from './apply-server-errors'
import { isServerValidationError } from './types'

export interface DocFormProviderProps<TForm extends FieldValues, TResponse> {

  schema: unknown
  defaultValues: DefaultValues<TForm>
  mutationFn: CompositeMutationFn<TForm, TResponse>
  onSuccess?: CompositeSuccessHandler<TResponse>

  onGlobalErrors?: (issues: ServerValidationIssue[]) => void

  formId?: string

  formApiRef?: MutableRefObject<UseFormReturn<TForm> | null>
  children: ReactNode
}

export function DocFormProvider<TForm extends FieldValues, TResponse>({
  schema,
  defaultValues,
  mutationFn,
  onSuccess,
  onGlobalErrors,
  formId,
  formApiRef,
  children,
}: DocFormProviderProps<TForm, TResponse>) {
  const form = useForm<TForm>({
    resolver: zodResolver(schema as never),
    defaultValues,
    mode: 'onBlur',
  })

  useEffect(() => {
    if (!formApiRef)
      return
    formApiRef.current = form
    return () => {
      formApiRef.current = null
    }
  }, [form, formApiRef])

  const [submitting, setSubmitting] = useState(false)

  const handleSubmit = form.handleSubmit(async (data) => {
    setSubmitting(true)

    try {
      const saved = await mutationFn(data)
      onSuccess?.(saved)
    }
    catch (err) {
      if (isServerValidationError(err)) {
        const globals = applyServerErrors(form, err)
        onGlobalErrors?.(globals)
      }
      else {
        onGlobalErrors?.([
          {
            field: '',
            code: 'serverUnreachable',
            message: 'forms.error.serverUnreachable',
          },
        ])
      }
    }
    finally {
      setSubmitting(false)
    }
  })

  return (
    <FormProvider {...form}>
      <form
        id={formId}
        onSubmit={handleSubmit}
        data-slot="doc-form"
        data-submitting={submitting || undefined}
        className="flex flex-1 min-h-0 flex-col"
      >
        {children}
      </form>
    </FormProvider>
  )
}
