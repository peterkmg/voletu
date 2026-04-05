import type { Column, Row, Table as TanstackTable } from '@tanstack/react-table'
import type { CSSProperties } from 'react'

export const alignClasses = {
  left: 'text-left',
  center: 'text-center',
  right: 'text-right',
}

/** Compute sticky positioning styles for pinned columns */
export function getPinningStyles<TData>(column: Column<TData, unknown>): CSSProperties {
  const isPinned = column.getIsPinned()
  if (!isPinned)
    return {}

  return {
    position: 'sticky',
    left: isPinned === 'left' ? `${column.getStart('left')}px` : undefined,
    right: isPinned === 'right' ? `${column.getAfter('right')}px` : undefined,
    zIndex: 1,
  }
}

export function hasAnyPinning<TData>(table: TanstackTable<TData>): boolean {
  return table.getLeftLeafColumns().length > 0 || table.getRightLeafColumns().length > 0
}

export function hasAnyFooter<TData>(table: TanstackTable<TData>): boolean {
  return table.getFooterGroups().some(group =>
    group.headers.some(header => header.column.columnDef.footer),
  )
}

/** Create a global filter function that searches across specified fields */
export function createGlobalFilter<T>(...fields: (keyof T & string)[]) {
  return (row: Row<T>, _columnId: string, filterValue: string) => {
    const search = String(filterValue).toLowerCase()
    return fields.some((field) => {
      const value = (row.original as Record<string, unknown>)[field]
      return String(value ?? '').toLowerCase().includes(search)
    })
  }
}

/** Compute CSS Grid column template from visible columns.
 *
 *  Priority: manual resize > explicit sizingCategory > legacy min/max heuristic.
 *  See ColumnMeta.sizingCategory for the three categories. */
export function getGridTemplate<TData>(table: TanstackTable<TData>): string {
  const sizing = table.getState().columnSizing
  return table.getVisibleLeafColumns().map((col) => {
    const { minSize, maxSize } = col.columnDef
    const category = col.columnDef.meta?.sizingCategory

    // 1. Manually resized column: user intent always wins
    if (sizing[col.id] != null)
      return `${col.getSize()}px`

    // 2. Explicit sizing category (preferred path)
    if (category === 'fixed')
      return `${col.getSize()}px`
    if (category === 'capped') {
      const min = minSize ?? 80
      const max = maxSize ?? 150
      return `minmax(${min}px, ${max}px)`
    }
    if (category === 'flex') {
      const min = minSize ?? 120
      return `minmax(${min}px, 1fr)`
    }

    // 3. Legacy fallback: infer from minSize/maxSize (backward compat)
    if (minSize != null && maxSize != null && minSize === maxSize)
      return `${col.getSize()}px`
    const min = minSize ?? 80
    const max = maxSize ? `${maxSize}px` : '1fr'
    return `minmax(${min}px, ${max})`
  }).join(' ')
}
