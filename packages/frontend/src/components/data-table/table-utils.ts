import type { Column, Row, Table as TanstackTable } from '@tanstack/react-table'
import type { CSSProperties } from 'react'

export const alignClasses = {
  left: 'text-left justify-start',
  center: 'text-center justify-center',
  right: 'text-right justify-end',
}

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

export function createGlobalFilter<T>(...fields: (keyof T & string)[]) {
  return (row: Row<T>, _columnId: string, filterValue: string) => {
    const search = String(filterValue).toLowerCase()

    return fields.some((field) => {
      const value = (row.original as Record<string, unknown>)[field]

      return String(value ?? '').toLowerCase().includes(search)
    })
  }
}

export function getGridTemplate<TData>(table: TanstackTable<TData>): string {
  const sizing = table.getState().columnSizing

  return table.getVisibleLeafColumns().map((col) => {
    const { minSize, maxSize } = col.columnDef
    const category = col.columnDef.meta?.sizingCategory

    if (sizing[col.id] != null)
      return `${col.getSize()}px`

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

    if (minSize != null && maxSize != null && minSize === maxSize)
      return `${col.getSize()}px`

    const min = minSize ?? 80
    const max = maxSize ? `${maxSize}px` : '1fr'

    return `minmax(${min}px, ${max})`
  }).join(' ')
}

export interface GroupInfo {
  isFirstOfGroup: boolean
  isLastOfGroup: boolean
  groupId: string
}

export function computeGroupInfo<T>(
  rows: Row<T>[],
  index: number,
  groupKey: string,
): GroupInfo {
  const currentVal = String((rows[index]!.original as Record<string, unknown>)[groupKey] ?? '')
  const prevVal = index > 0 ? String((rows[index - 1]!.original as Record<string, unknown>)[groupKey] ?? '') : null
  const nextVal = index < rows.length - 1 ? String((rows[index + 1]!.original as Record<string, unknown>)[groupKey] ?? '') : null

  return {
    isFirstOfGroup: prevVal !== currentVal,
    isLastOfGroup: nextVal !== currentVal,
    groupId: currentVal,
  }
}
