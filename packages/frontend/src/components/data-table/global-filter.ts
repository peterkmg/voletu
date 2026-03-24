import type { Row } from '@tanstack/react-table'

export function createGlobalFilter<T>(...fields: (keyof T & string)[]) {
  return (row: Row<T>, _columnId: string, filterValue: string) => {
    const search = String(filterValue).toLowerCase()
    return fields.some((field) => {
      const value = (row.original as Record<string, unknown>)[field]
      return String(value ?? '').toLowerCase().includes(search)
    })
  }
}
