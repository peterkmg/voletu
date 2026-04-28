import type { ReactNode } from 'react'
import type { ColumnSpec, RowFieldSpec } from '~/components/composite-form/types'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { FormProvider, useForm } from 'react-hook-form'
import { I18nextProvider } from 'react-i18next'
import { describe, expect, it } from 'vitest'
import { z } from 'zod'
import { DocItemsTable } from '~/components/composite-form/doc-items-table'
import i18n from '~/i18n/config'

interface Item {
  product: string
  qty: number
}

const itemSchema = z.object({
  product: z.string().min(1),
  qty: z.number().positive(),
})

const columns: ColumnSpec<Item>[] = [
  { key: 'product', labelKey: 'product' },
  { key: 'qty', labelKey: 'qty' },
]

function TextInput({ field }: { field: { name: string, value: unknown } }) {
  return <input data-testid={field.name} />
}

const fields: RowFieldSpec<Item>[] = [
  { name: 'product', labelKey: 'product', component: TextInput },
  { name: 'qty', labelKey: 'qty', component: TextInput },
]

const emptyRow: Item = { product: '', qty: 0 }

function Wrapper({ children }: { children: ReactNode }) {
  const form = useForm<{ items: Item[] }>({
    defaultValues: { items: [{ product: 'A', qty: 1 }] },
  })
  return (
    <I18nextProvider i18n={i18n}>
      <FormProvider {...form}>{children}</FormProvider>
    </I18nextProvider>
  )
}

describe('docItemsTable', () => {
  it('renders a row for each item in the field array', () => {
    render(
      <Wrapper>
        <DocItemsTable
          name="items"
          columns={columns}
          rowSchema={itemSchema}
          rowFields={fields}
          emptyRow={emptyRow}
        />
      </Wrapper>,
    )
    // Both the md+ table and the < md card-list render the same data,
    // so each value appears twice in the DOM under jsdom.
    expect(screen.getAllByText('A').length).toBeGreaterThan(0)
    expect(screen.getAllByText('1').length).toBeGreaterThan(0)
  })

  it('opens the row drawer when "Add item" is clicked', async () => {
    const user = userEvent.setup()
    render(
      <Wrapper>
        <DocItemsTable
          name="items"
          columns={columns}
          rowSchema={itemSchema}
          rowFields={fields}
          emptyRow={emptyRow}
        />
      </Wrapper>,
    )
    await user.click(screen.getByRole('button', { name: /add/i }))
    expect(
      document.querySelector('[data-slot="doc-item-row-drawer"]'),
    ).toBeInTheDocument()
  })

  it('renders the empty-state caption when no items exist', () => {
    function EmptyWrapper({ children }: { children: ReactNode }) {
      const form = useForm<{ items: Item[] }>({ defaultValues: { items: [] } })
      return (
        <I18nextProvider i18n={i18n}>
          <FormProvider {...form}>{children}</FormProvider>
        </I18nextProvider>
      )
    }
    render(
      <EmptyWrapper>
        <DocItemsTable
          name="items"
          columns={columns}
          rowSchema={itemSchema}
          rowFields={fields}
          emptyRow={emptyRow}
        />
      </EmptyWrapper>,
    )
    // The empty state renders its translated caption from the forms namespace —
    // once inside the md+ table (TableCell colspan) and once in the < md card-list.
    const emptyCaptions = screen.getAllByText(/no items yet/i)
    expect(emptyCaptions.length).toBeGreaterThan(0)
  })
})
