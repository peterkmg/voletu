import type { Row } from '@tanstack/react-table'

export type FilterType = 'text' | 'date' | 'number' | 'enum'

const ISO_DATE_PREFIX = /^\d{4}-\d{2}-\d{2}/

export function detectFilterType(facetedValues: Map<unknown, number>): FilterType {
  for (const [value] of facetedValues) {
    if (value == null)
      continue
    if (typeof value === 'number')
      return 'number'
    if (typeof value === 'string') {
      if (ISO_DATE_PREFIX.test(value))
        return 'date'
    }
    break
  }
  return 'text'
}

// Filter conventions for the in-header column filter UI:
//   undefined → no filter (all rows match)
//   []        → every option deselected (no rows match)
//   [v, ...]  → match rows whose column value is one of the listed values

export function textArrayFilter<TData>(
  row: Row<TData>,
  columnId: string,
  filterValue: unknown,
): boolean {
  if (filterValue === undefined)
    return true
  if (!Array.isArray(filterValue))
    return true
  if (filterValue.length === 0)
    return false
  const val = row.getValue(columnId)
  if (val == null)
    return false
  return filterValue.includes(String(val))
}

export function dateArrayFilter<TData>(
  row: Row<TData>,
  columnId: string,
  filterValue: unknown,
): boolean {
  if (filterValue === undefined)
    return true
  if (!Array.isArray(filterValue))
    return true
  if (filterValue.length === 0)
    return false
  const val = row.getValue(columnId)
  if (val == null)
    return false
  return filterValue.includes(String(val).slice(0, 10))
}

export function numberRangeFilter<TData>(
  row: Row<TData>,
  columnId: string,
  filterValue: unknown,
): boolean {
  if (filterValue === undefined)
    return true
  if (!Array.isArray(filterValue))
    return true
  const [min, max] = filterValue as [number | undefined, number | undefined]
  if (min == null && max == null)
    return true
  const val = row.getValue<number | null | undefined>(columnId)
  if (val == null)
    return false
  if (min != null && val < min)
    return false
  if (max != null && val > max)
    return false
  return true
}
