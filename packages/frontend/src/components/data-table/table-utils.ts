import type { Column, Table as TanstackTable } from '@tanstack/react-table'
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

/** Compute CSS Grid column template from visible columns */
export function getGridTemplate<TData>(table: TanstackTable<TData>): string {
  const sizing = table.getState().columnSizing
  return table.getVisibleLeafColumns().map((col) => {
    const { minSize, maxSize } = col.columnDef
    // Fixed-width column (e.g. select, actions): minSize === maxSize
    if (minSize != null && maxSize != null && minSize === maxSize)
      return `${col.getSize()}px`
    // Manually resized column: use exact size
    if (sizing[col.id] != null)
      return `${col.getSize()}px`
    // Flexible column: grows to fill available space
    const min = minSize ?? 80
    return `minmax(${min}px, 1fr)`
  }).join(' ')
}
