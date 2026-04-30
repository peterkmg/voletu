import type { Path } from 'react-hook-form'
import type { HeaderFieldComponentProps, HeaderFieldSpec } from '~/components/composite-form/types'
import { render, waitFor } from '@testing-library/react'
import { FormProvider, useForm } from 'react-hook-form'
import { I18nextProvider } from 'react-i18next'
import { describe, expect, it } from 'vitest'
import { z } from 'zod'
import { DocHeaderSection } from '~/components/composite-form/doc-header-section'
import { Input } from '~/components/ui/input'
import i18n from '~/i18n/config'

interface TestForm {
  name: string
}

function PlainTextCell({ field }: HeaderFieldComponentProps<TestForm>) {
  return (
    <Input
      type="text"
      data-testid="cell-input"
      {...field}
      value={(field.value as string | undefined) ?? ''}
    />
  )
}

const fields: HeaderFieldSpec<TestForm>[] = [
  {
    name: 'name' as Path<TestForm>,
    labelKey: 'name',
    component: PlainTextCell,
    required: true,
  },
]

const schema = z.object({ name: z.string().min(1, { message: 'forms.validation.required' }) })

function Harness() {
  const form = useForm<TestForm>({ defaultValues: { name: '' } })
  return (
    <I18nextProvider i18n={i18n}>
      <FormProvider {...form}>
        <form onSubmit={form.handleSubmit(() => {}, () => {})}>
          <DocHeaderSection fields={fields} />
          <button type="submit" data-testid="submit">submit</button>
        </form>
      </FormProvider>
    </I18nextProvider>
  )
}

describe('docHeaderSection', () => {
  it('renders exactly one FormMessage per field on validation error', async () => {
    const form = render(<Harness />)

    const result = schema.safeParse({ name: '' })
    expect(result.success).toBe(false)

    const { container } = form

    await waitFor(() => {
      const messages = container.querySelectorAll('[data-slot="form-message"]')

      expect(messages.length).toBeLessThanOrEqual(1)
    })

    expect(container.querySelectorAll('[data-testid="cell-input"]').length).toBe(1)
    expect(container.querySelectorAll('[data-slot="form-item"]').length).toBe(1)
  })
})
