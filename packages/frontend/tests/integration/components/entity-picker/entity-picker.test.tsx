import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { EntityPickerCombobox, EntityPickerDialog } from '~/components/entity-picker'

const duplicateItems = [
  { id: 'company-1', label: 'Acme Logistics', secondary: 'Budapest' },
  { id: 'company-2', label: 'Acme Logistics', secondary: 'Budapest' },
  { id: 'company-3', label: 'Northern Depot', secondary: 'Szeged' },
]

function highlightedCommandItems() {
  return Array.from(document.querySelectorAll('[cmdk-item][data-selected="true"]'))
}

function commandInput() {
  return document.querySelector<HTMLInputElement>('[cmdk-input]')!
}

describe('entity picker menus', () => {
  it('highlights only one combobox item when display names are duplicated', async () => {
    const user = userEvent.setup()

    render(
      <EntityPickerCombobox
        items={duplicateItems}
        value={null}
        onChange={vi.fn()}
      />,
    )

    await user.click(screen.getByRole('combobox'))

    expect(highlightedCommandItems()).toHaveLength(1)
  })

  it('keeps duplicate combobox items searchable by their visible label', async () => {
    const user = userEvent.setup()

    render(
      <EntityPickerCombobox
        items={duplicateItems}
        value={null}
        onChange={vi.fn()}
      />,
    )

    await user.click(screen.getByRole('combobox'))
    await user.type(commandInput(), 'Acme')

    expect(screen.getAllByText('Acme Logistics')).toHaveLength(2)
    expect(screen.queryByText('Northern Depot')).not.toBeInTheDocument()
  })

  it('highlights only one dialog item when display names are duplicated', () => {
    render(
      <EntityPickerDialog
        open
        onOpenChange={vi.fn()}
        items={duplicateItems}
        value={null}
        onSelect={vi.fn()}
        title="Select company"
      />,
    )

    expect(highlightedCommandItems()).toHaveLength(1)
  })

  it('keeps duplicate dialog items searchable by their visible label and secondary text', async () => {
    const user = userEvent.setup()

    render(
      <EntityPickerDialog
        open
        onOpenChange={vi.fn()}
        items={duplicateItems}
        value={null}
        onSelect={vi.fn()}
        title="Select company"
      />,
    )

    await user.type(commandInput(), 'Budapest')

    const list = screen.getByRole('listbox')
    expect(within(list).getAllByText('Acme Logistics')).toHaveLength(2)
    expect(within(list).queryByText('Northern Depot')).not.toBeInTheDocument()
  })
})
