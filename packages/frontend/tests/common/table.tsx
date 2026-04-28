import type { ColumnDef, TableOptions } from '@tanstack/react-table'
import {
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from '@tanstack/react-table'

export interface TestItem {
  id: string
  name: string
  status: string
  createdAt: string
}

export function createTestData(count: number): TestItem[] {
  return Array.from({ length: count }, (_, i) => ({
    id: `id-${i + 1}`,
    name: `Item ${i + 1}`,
    status: i % 2 === 0 ? 'active' : 'inactive',
    createdAt: '2026-03-24',
  }))
}

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
