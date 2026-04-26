// packages/frontend/src/views/dashboard/__tests__/matrix-toolbar.test.tsx
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it } from 'vitest'
import { MatrixToolbar } from '../components/matrix-toolbar'
import { useDashboardStore } from '../state/dashboard-store'

function resetStore() {
  useDashboardStore.setState({
    contractorId: null,
    orientation: 'products-as-rows',
    showType: false,
    showBase: false,
    productGroupTotals: false,
    productTypeTotals: false,
    warehouseTotals: false,
    baseTotals: false,
  })
}

describe('matrixToolbar', () => {
  it('flips the Base switch inside the visibility popover and updates the store', async () => {
    resetStore()
    const user = userEvent.setup()
    render(
      <MatrixToolbar
        contractors={[{ id: 'c1', label: 'Acme' }]}
        searchQuery=""
        onSearchChange={() => {}}
      />,
    )
    await user.click(screen.getByRole('button', { name: /visibility/i }))
    const baseSwitch = await screen.findByLabelText(/show base/i)
    await user.click(baseSwitch)
    expect(useDashboardStore.getState().showBase).toBe(true)
  })

  it('shows compact rows toggle and switches orientation when storages tab is selected', async () => {
    resetStore()
    const user = userEvent.setup()
    render(
      <MatrixToolbar
        contractors={[{ id: 'c1', label: 'Acme' }]}
        searchQuery=""
        onSearchChange={() => {}}
      />,
    )
    expect(screen.getByText(/^rows:$/i)).toBeInTheDocument()
    await user.click(screen.getByRole('tab', { name: /^storages$/i }))
    expect(useDashboardStore.getState().orientation).toBe('storages-as-rows')
  })

  it('disables productType totals when showType is off', async () => {
    resetStore()
    const user = userEvent.setup()
    render(
      <MatrixToolbar
        contractors={[{ id: 'c1', label: 'Acme' }]}
        searchQuery=""
        onSearchChange={() => {}}
      />,
    )
    await user.click(screen.getByRole('button', { name: /visibility/i }))
    const productTypeTotalsSwitch = await screen.findByLabelText(/product type totals/i)
    expect(productTypeTotalsSwitch).toBeDisabled()
  })

  it('uses one combined visibility popover instead of separate structure and totals popovers', () => {
    resetStore()
    render(
      <MatrixToolbar
        contractors={[{ id: 'c1', label: 'Acme' }]}
        searchQuery=""
        onSearchChange={() => {}}
      />,
    )
    expect(screen.getByRole('button', { name: /visibility/i })).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /structure/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /totals/i })).not.toBeInTheDocument()
  })
})
