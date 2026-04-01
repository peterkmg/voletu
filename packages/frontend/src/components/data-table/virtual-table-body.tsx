import type { ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import { useVirtualizer } from '@tanstack/react-virtual'
import { useCallback, useRef } from 'react'
import { TableBody } from '~/components/ui/table'
import { EmptyState } from './empty-state'
import { DataRow } from './table-data-row'
import { TableSkeleton } from './table-skeleton'

interface VirtualTableBodyProps<TData> {
  table: TanstackTable<TData>
  columns: ColumnDef<TData, unknown>[]
  /** The scrollable container element (managed by parent DataTable via useState) */
  scrollElement: HTMLDivElement | null
  isPinning: boolean
  densityCls: string
  overscan?: number
  estimateSize?: number
  isLoading?: boolean
  emptyMessage?: string
  emptyIcon?: React.ReactNode
  onRowAction?: (row: TData) => void
}

export function VirtualTableBody<TData>({
  table,
  columns,
  scrollElement,
  isPinning,
  densityCls,
  overscan = 20,
  estimateSize = 40,
  isLoading,
  emptyMessage,
  emptyIcon,
  onRowAction,
}: VirtualTableBodyProps<TData>) {
  const { rows } = table.getRowModel()

  // Ref keeps rows accessible in callbacks without causing re-creation
  const rowsRef = useRef(rows)
  rowsRef.current = rows

  const virtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => scrollElement,
    estimateSize: () => estimateSize,
    overscan,
  })

  const virtualRows = virtualizer.getVirtualItems()
  const totalSize = virtualizer.getTotalSize()

  // Focus a virtual row, scrolling if needed
  const focusVirtualRow = useCallback((targetIndex: number) => {
    if (targetIndex < 0 || targetIndex >= rowsRef.current.length)
      return
    const el = scrollElement?.querySelector<HTMLElement>(
      `[data-virtual-index="${targetIndex}"]`,
    )
    if (el) {
      el.focus({ preventScroll: true })
      el.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
    }
    else {
      virtualizer.scrollToIndex(targetIndex, { align: 'auto' })
      requestAnimationFrame(() => {
        const target = scrollElement?.querySelector<HTMLElement>(
          `[data-virtual-index="${targetIndex}"]`,
        )
        target?.focus({ preventScroll: true })
      })
    }
  }, [virtualizer, scrollElement])

  // Keyboard navigation – stable callback (reads rows from ref)
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>, rowData: TData, virtualIndex: number) => {
      switch (e.key) {
        case 'ArrowDown': {
          e.preventDefault()
          focusVirtualRow(virtualIndex + 1)
          break
        }
        case 'ArrowUp': {
          e.preventDefault()
          focusVirtualRow(virtualIndex - 1)
          break
        }
        case 'Home': {
          e.preventDefault()
          focusVirtualRow(0)
          break
        }
        case 'End': {
          e.preventDefault()
          focusVirtualRow(rowsRef.current.length - 1)
          break
        }
        case 'PageDown': {
          e.preventDefault()
          const visibleCount = virtualizer.getVirtualItems().length - overscan * 2
          const pageSize = Math.max(1, Math.floor(visibleCount))
          focusVirtualRow(Math.min(virtualIndex + pageSize, rowsRef.current.length - 1))
          break
        }
        case 'PageUp': {
          e.preventDefault()
          const visibleCount = virtualizer.getVirtualItems().length - overscan * 2
          const pageSize = Math.max(1, Math.floor(visibleCount))
          focusVirtualRow(Math.max(virtualIndex - pageSize, 0))
          break
        }
        case 'Enter': {
          e.preventDefault()
          onRowAction?.(rowData)
          break
        }
        case ' ': {
          e.preventDefault()
          rowsRef.current[virtualIndex]?.toggleSelected()
          break
        }
      }
    },
    [onRowAction, virtualizer, focusVirtualRow, overscan],
  )

  if (rows.length === 0) {
    return (
      <TableBody>
        {isLoading
          ? <TableSkeleton columns={columns.length} densityCls={densityCls} />
          : <EmptyState colSpan={columns.length} message={emptyMessage} icon={emptyIcon} />}
      </TableBody>
    )
  }

  return (
    <TableBody
      style={{ position: 'relative', height: `${totalSize}px` }}
    >
      {virtualRows.map((virtualRow) => {
        const row = rows[virtualRow.index]!
        return (
          <DataRow
            key={row.id}
            row={row}
            rowIndex={virtualRow.index}
            isPinning={isPinning}
            densityCls={densityCls}
            onKeyDown={handleKeyDown}
            onRowAction={onRowAction}
            measureRef={virtualizer.measureElement}
            virtualStart={virtualRow.start}
          />
        )
      })}
    </TableBody>
  )
}
