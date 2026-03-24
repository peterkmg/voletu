import type { ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import { flexRender } from '@tanstack/react-table'
import { useVirtualizer } from '@tanstack/react-virtual'
import { useCallback, useRef } from 'react'
import {
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { densityClasses, useDensity } from './density-context'
import { EmptyState } from './empty-state'
import { TableSkeleton } from './table-skeleton'
import { alignClasses, getPinningStyles, hasAnyFooter, hasAnyPinning } from './table-utils'

interface VirtualizedDataTableProps<TData> {
  table: TanstackTable<TData>
  columns: ColumnDef<TData, unknown>[]
  className?: string
  /** Height of the scrollable container. Default: '600px' */
  height?: string
  /** Estimated row height in pixels. Default: 40 */
  estimateSize?: number
  /** Number of rows to render outside visible area. Default: 20 */
  overscan?: number
  /** Show skeleton loading state. */
  isLoading?: boolean
  /** Custom empty state message. */
  emptyMessage?: string
  /** Custom empty state icon. */
  emptyIcon?: React.ReactNode
  /** Called when a row receives Enter key or is double-clicked. */
  onRowAction?: (row: TData) => void
}

export function VirtualizedDataTable<TData>({
  table,
  columns,
  className,
  height = '600px',
  estimateSize = 40,
  overscan = 20,
  isLoading,
  emptyMessage,
  emptyIcon,
  onRowAction,
}: VirtualizedDataTableProps<TData>) {
  const { density } = useDensity()
  const densityCls = densityClasses[density]
  const containerRef = useRef<HTMLDivElement>(null)

  const isPinning = hasAnyPinning(table)
  const showFooter = hasAnyFooter(table)

  const { rows } = table.getRowModel()

  const virtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => containerRef.current,
    estimateSize: () => estimateSize,
    overscan,
  })

  const virtualRows = virtualizer.getVirtualItems()
  const totalSize = virtualizer.getTotalSize()

  // Focus a virtual row, scrolling the virtualizer if the row isn't rendered yet
  const focusVirtualRow = useCallback((targetIndex: number) => {
    if (targetIndex < 0 || targetIndex >= rows.length)
      return
    const el = containerRef.current?.querySelector<HTMLElement>(
      `[data-virtual-index="${targetIndex}"]`,
    )
    if (el) {
      el.focus({ preventScroll: true })
      el.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
    }
    else {
      virtualizer.scrollToIndex(targetIndex, { align: 'auto' })
      requestAnimationFrame(() => {
        const target = containerRef.current?.querySelector<HTMLElement>(
          `[data-virtual-index="${targetIndex}"]`,
        )
        target?.focus({ preventScroll: true })
      })
    }
  }, [rows.length, virtualizer])

  // Keyboard navigation
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTableRowElement>, rowData: TData, virtualIndex: number) => {
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
          focusVirtualRow(rows.length - 1)
          break
        }
        case 'PageDown': {
          e.preventDefault()
          const visibleCount = virtualizer.getVirtualItems().length - overscan * 2
          const pageSize = Math.max(1, Math.floor(visibleCount))
          focusVirtualRow(Math.min(virtualIndex + pageSize, rows.length - 1))
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
          rows[virtualIndex]?.toggleSelected()
          break
        }
      }
    },
    [rows, onRowAction, virtualizer, focusVirtualRow, overscan],
  )

  return (
    <div
      ref={containerRef}
      className={cn('overflow-auto rounded-md border', className)}
      style={{ height }}
    >
      <Table className="min-w-xl">
        <TableHeader className="sticky top-0 z-10 bg-background">
          {table.getHeaderGroups().map(headerGroup => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                const meta = header.column.columnDef.meta
                const alignCls = meta?.align ? alignClasses[meta.align] : ''
                const pinStyle = isPinning ? getPinningStyles(header.column) : {}
                const pinnedSide = header.column.getIsPinned()
                const pinBg = pinnedSide ? 'bg-background' : ''
                return (
                  <TableHead
                    key={header.id}
                    colSpan={header.colSpan}
                    className={cn(meta?.className, meta?.thClassName, alignCls, pinBg)}
                    aria-label={pinnedSide ? `Pinned ${pinnedSide} column` : undefined}
                    style={{
                      ...pinStyle,
                      width: header.getSize(),
                    }}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(header.column.columnDef.header, header.getContext())}
                  </TableHead>
                )
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {rows.length === 0
            ? (isLoading
                ? <TableSkeleton columns={columns.length} densityCls={densityCls} />
                : <EmptyState colSpan={columns.length} message={emptyMessage} icon={emptyIcon} />
              )
            : (
                <>
                  {/* Spacer row for virtual offset */}
                  {virtualRows.length > 0 && virtualRows[0]!.start > 0 && (
                    <tr style={{ height: `${virtualRows[0]!.start}px` }} />
                  )}
                  {virtualRows.map((virtualRow) => {
                    const row = rows[virtualRow.index]!
                    return (
                      <TableRow
                        key={row.id}
                        data-state={row.getIsSelected() && 'selected'}
                        data-virtual-index={virtualRow.index}
                        ref={virtualizer.measureElement}
                        data-index={virtualRow.index}
                        tabIndex={0}
                        onKeyDown={e => handleKeyDown(e, row.original, virtualRow.index)}
                        onDoubleClick={() => onRowAction?.(row.original)}
                        className="focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                      >
                        {row.getVisibleCells().map((cell) => {
                          const meta = cell.column.columnDef.meta
                          const alignCls = meta?.align ? alignClasses[meta.align] : ''
                          const pinStyle = isPinning ? getPinningStyles(cell.column) : {}
                          const pinBg = cell.column.getIsPinned() ? 'bg-background' : ''
                          return (
                            <TableCell
                              key={cell.id}
                              className={cn(meta?.className, meta?.tdClassName, alignCls, densityCls, pinBg)}
                              style={{
                                ...pinStyle,
                                width: cell.column.getSize(),
                              }}
                            >
                              {flexRender(cell.column.columnDef.cell, cell.getContext())}
                            </TableCell>
                          )
                        })}
                      </TableRow>
                    )
                  })}
                  {/* Spacer row for remaining virtual space */}
                  {virtualRows.length > 0
                    && virtualRows[virtualRows.length - 1]!.end < totalSize && (
                    <tr
                      style={{
                        height: `${totalSize - virtualRows[virtualRows.length - 1]!.end}px`,
                      }}
                    />
                  )}
                </>
              )}
        </TableBody>
        {showFooter && (
          <TableFooter className="sticky bottom-0 z-10">
            {table.getFooterGroups().map(footerGroup => (
              <TableRow key={footerGroup.id}>
                {footerGroup.headers.map((header) => {
                  const meta = header.column.columnDef.meta
                  const alignCls = meta?.align ? alignClasses[meta.align] : ''
                  return (
                    <TableCell
                      key={header.id}
                      colSpan={header.colSpan}
                      className={cn(alignCls, 'font-medium')}
                    >
                      {header.isPlaceholder
                        ? null
                        : flexRender(header.column.columnDef.footer, header.getContext())}
                    </TableCell>
                  )
                })}
              </TableRow>
            ))}
          </TableFooter>
        )}
      </Table>
    </div>
  )
}
