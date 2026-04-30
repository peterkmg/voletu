import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { DashboardView } from '~/views/dashboard/index'

vi.mock('~/components/layout/header', () => ({ Header: () => null }))
vi.mock('~/components/layout/main', () => ({
  Main: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}))

vi.mock('~/views/dashboard/components/empty-states', () => ({
  EmptyState: (_props: { variant: string }) => (
    <div role="heading">no contractors</div>
  ),
}))

vi.mock('~/views/dashboard/hooks/use-inventory-matrix-data', () => ({
  useInventoryMatrixData: () => ({
    vm: null,
    contractors: [],
    isLoading: false,
    isError: false,
    error: null,
    hasAnyData: true,
    refetchAll: () => {},
  }),
}))

describe('dashboardView', () => {
  it('renders the no-contractors empty state when no contractors exist', () => {
    render(<DashboardView />)
    expect(screen.getByRole('heading', { name: /no contractors/i })).toBeInTheDocument()
  })
})
