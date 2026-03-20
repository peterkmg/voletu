import type { HeaderContext } from '@tanstack/react-table'

/** Sum all numeric values in a column for the filtered rows */
export function sumFooter<TData, TValue>(
  info: HeaderContext<TData, TValue>,
): string {
  const total = info.table
    .getFilteredRowModel()
    .rows
    .reduce((sum, row) => {
      const val = row.getValue<number>(info.column.id)
      return sum + (typeof val === 'number' ? val : 0)
    }, 0)
  return total.toLocaleString(undefined, { maximumFractionDigits: 2 })
}

/** Average all numeric values in a column for the filtered rows */
export function averageFooter<TData, TValue>(
  info: HeaderContext<TData, TValue>,
): string {
  const rows = info.table.getFilteredRowModel().rows
  if (rows.length === 0)
    return '—'
  const total = rows.reduce((sum, row) => {
    const val = row.getValue<number>(info.column.id)
    return sum + (typeof val === 'number' ? val : 0)
  }, 0)
  return (total / rows.length).toLocaleString(undefined, { maximumFractionDigits: 2 })
}

/** Count of filtered rows (useful for first column) */
export function countFooter<TData, TValue>(
  info: HeaderContext<TData, TValue>,
): string {
  return `${info.table.getFilteredRowModel().rows.length} rows`
}
