import { render } from '@testing-library/react'
import { I18nextProvider } from 'react-i18next'
import { z } from 'zod'
import { CompositeFormDialog } from '~/components/composite-form/composite-form-dialog'
import i18n from '~/i18n/config'

describe('compositeFormDialog', () => {
  it('uses a flex + overflow-hidden shell so footer stays inside viewport', () => {
    render(
      <I18nextProvider i18n={i18n}>
        <CompositeFormDialog<{ name: string }, unknown>
          open
          onOpenChange={() => {}}
          mode="create"
          schema={z.object({ name: z.string() })}
          defaultValues={{ name: '' }}
          mutationFn={async () => ({})}
          titleKey="acceptance:dialog.title.create"
          descriptionKey="acceptance:dialog.title.edit"
        >
          <div className="h-[1200px]">Tall content</div>
        </CompositeFormDialog>
      </I18nextProvider>,
    )

    const dialog = document.querySelector('[data-slot="composite-form-dialog"]')
    const form = document.querySelector('[data-slot="doc-form"]')
    const body = document.querySelector('[data-slot="composite-form-body"]')
    expect(dialog).toHaveClass('flex')
    expect(dialog).toHaveClass('flex-col')
    expect(dialog).toHaveClass('overflow-hidden')
    expect(form).toHaveClass('flex')
    expect(form).toHaveClass('flex-col')
    expect(form).toHaveClass('min-h-0')
    expect(form).toHaveClass('flex-1')
    expect(body).toHaveClass('min-h-0')
    expect(body).toHaveClass('flex-1')
    expect(body).toHaveClass('overflow-y-auto')
  })
})
