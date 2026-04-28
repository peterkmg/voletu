import { render, screen } from '@testing-library/react'
import { FormProvider, useForm } from 'react-hook-form'
import { I18nextProvider } from 'react-i18next'
import { describe, expect, it } from 'vitest'
import { FormControl, FormField, FormItem, FormMessage } from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import i18n from '~/i18n/config'

interface TestFormValues {
  name: string
}

function FormMessageHarness({ message }: { message: string }) {
  const form = useForm<TestFormValues>({
    defaultValues: { name: '' },
    errors: {
      name: { type: 'server', message },
    },
  })

  return (
    <I18nextProvider i18n={i18n}>
      <FormProvider {...form}>
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <Input {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
      </FormProvider>
    </I18nextProvider>
  )
}

describe('formMessage', () => {
  it('translates forms.validation message keys', () => {
    render(<FormMessageHarness message="forms.validation.required" />)

    expect(screen.getByText('Required')).toBeInTheDocument()
  })

  it('keeps arbitrary server messages unchanged', () => {
    render(<FormMessageHarness message="Custom server message" />)

    expect(screen.getByText('Custom server message')).toBeInTheDocument()
  })

  it('keeps non-validation forms namespace messages unchanged', () => {
    render(<FormMessageHarness message="forms.cancel" />)

    expect(screen.getByText('forms.cancel')).toBeInTheDocument()
  })
})
