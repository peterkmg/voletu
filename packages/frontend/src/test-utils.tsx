import type { ColumnDef, TableOptions } from '@tanstack/react-table'
import type { RenderOptions } from '@testing-library/react'
import type { ReactElement } from 'react'
import {
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from '@tanstack/react-table'
import { render } from '@testing-library/react'
import { DensityProvider } from '~/components/data-table/density'

function TestProviders({ children }: { children: React.ReactNode }) {
  return (
    <DensityProvider>
      {children}
    </DensityProvider>
  )
}

/** Render with DensityProvider and other required providers */
export function renderWithProviders(ui: ReactElement, options?: Omit<RenderOptions, 'wrapper'>) {
  return render(ui, { wrapper: TestProviders, ...options })
}

export { render } from '@testing-library/react'
export { default as userEvent } from '@testing-library/user-event'

/**
 * Hook wrapper that creates a TanStack Table instance for testing.
 * Must be called inside a React component.
 */
export function useTestTable<T>(
  data: T[],
  columns: ColumnDef<T, unknown>[],
  overrides?: Partial<TableOptions<T>>,
) {
  return useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    ...overrides,
  })
}

/** Simple test data type */
export interface TestItem {
  id: string
  name: string
  status: string
  createdAt: string
}

/** Generate test data */
export function createTestData(count: number): TestItem[] {
  return Array.from({ length: count }, (_, i) => ({
    id: `id-${i + 1}`,
    name: `Item ${i + 1}`,
    status: i % 2 === 0 ? 'active' : 'inactive',
    createdAt: '2026-03-24',
  }))
}
