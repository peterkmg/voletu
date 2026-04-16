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
  it('flips the Base switch inside the Structure popover and updates the store', async () => {
    resetStore()
    const user = userEvent.setup()
    render(
      <MatrixToolbar
        contractors={[{ id: 'c1', label: 'Acme' }]}
        searchQuery=""
        onSearchChange={() => {}}
      />,
    )
    await user.click(screen.getByRole('button', { name: /structure/i }))
    const baseSwitch = await screen.findByLabelText(/show base/i)
    await user.click(baseSwitch)
    expect(useDashboardStore.getState().showBase).toBe(true)
  })

  it('switches orientation when storages-as-rows tab is selected', async () => {
    resetStore()
    const user = userEvent.setup()
    render(
      <MatrixToolbar
        contractors={[{ id: 'c1', label: 'Acme' }]}
        searchQuery=""
        onSearchChange={() => {}}
      />,
    )
    await user.click(screen.getByRole('tab', { name: /storages as rows/i }))
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
    await user.click(screen.getByRole('button', { name: /totals/i }))
    const productTypeTotalsSwitch = await screen.findByLabelText(/product type totals/i)
    expect(productTypeTotalsSwitch).toBeDisabled()
  })
})
