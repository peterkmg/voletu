import type { ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import { flexRender } from '@tanstack/react-table'
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

interface DataTableProps<TData> {
  table: TanstackTable<TData>
  columns: ColumnDef<TData, unknown>[]
  className?: string
  /** Enable sticky column headers when scrolling vertically. Default: true */
  stickyHeader?: boolean
  /** Max height for scrollable table container. Enables vertical scrolling. */
  maxHeight?: string
  /** Show skeleton loading state. */
  isLoading?: boolean
  /** Custom empty state message. */
  emptyMessage?: string
  /** Custom empty state icon. */
  emptyIcon?: React.ReactNode
  /** Called when a row receives Enter key or is double-clicked. */
  onRowAction?: (row: TData) => void
}

export function DataTable<TData>({
  table,
  columns,
  className,
  stickyHeader = true,
  maxHeight,
  isLoading,
  emptyMessage,
  emptyIcon,
  onRowAction,
}: DataTableProps<TData>) {
  const { density } = useDensity()
  const densityCls = densityClasses[density]
  const tableRef = useRef<HTMLDivElement>(null)

  const isPinning = hasAnyPinning(table)
  const showFooter = hasAnyFooter(table)
  const isResizing = table.getState().columnSizingInfo.isResizingColumn

  // Focus a row without scrolling the document (prevents layout break in fixed layout)
  const focusRow = useCallback((el: HTMLElement | null | undefined) => {
    if (!el)
      return
    el.focus({ preventScroll: true })
    el.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
  }, [])

  // Keyboard navigation handler
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTableRowElement>, rowData: TData, rowIndex: number) => {
      const rows = table.getRowModel().rows
      switch (e.key) {
        case 'ArrowDown': {
          e.preventDefault()
          focusRow(tableRef.current?.querySelector(`[data-row-index="${rowIndex + 1}"]`))
          break
        }
        case 'ArrowUp': {
          e.preventDefault()
          focusRow(tableRef.current?.querySelector(`[data-row-index="${rowIndex - 1}"]`))
          break
        }
        case 'Home': {
          e.preventDefault()
          focusRow(tableRef.current?.querySelector('[data-row-index="0"]'))
          break
        }
        case 'End': {
          e.preventDefault()
          focusRow(tableRef.current?.querySelector(`[data-row-index="${rows.length - 1}"]`))
          break
        }
        case 'Enter': {
          e.preventDefault()
          onRowAction?.(rowData)
          break
        }
        case ' ': {
          e.preventDefault()
          const row = rows[rowIndex]
          row?.toggleSelected()
          break
        }
      }
    },
    [table, onRowAction, focusRow],
  )

  return (
    <div
      ref={tableRef}
      className={cn(
        'overflow-auto rounded-md border',
        className,
      )}
      style={maxHeight ? { maxHeight } : undefined}
    >
      <Table
        className="min-w-xl"
        style={isResizing ? { userSelect: 'none' } : undefined}
      >
        <TableHeader className={stickyHeader ? 'sticky top-0 z-10 bg-background' : undefined}>
          {table.getHeaderGroups().map(headerGroup => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                const meta = header.column.columnDef.meta
                const alignCls = meta?.align ? alignClasses[meta.align] : ''
                const pinStyle = isPinning ? getPinningStyles(header.column) : {}
                const pinBg = header.column.getIsPinned() ? 'bg-background' : ''

                const pinnedSide = header.column.getIsPinned()
                return (
                  <TableHead
                    key={header.id}
                    colSpan={header.colSpan}
                    className={cn(meta?.className, meta?.thClassName, alignCls, pinBg)}
                    aria-label={pinnedSide ? `Pinned ${pinnedSide} column` : undefined}
                    style={{
                      ...pinStyle,
                      width: header.column.getCanResize() ? header.getSize() : undefined,
                    }}
                  >
                    <div className="flex items-center">
                      <div className="flex-1">
                        {header.isPlaceholder
                          ? null
                          : flexRender(header.column.columnDef.header, header.getContext())}
                      </div>
                      {header.column.getCanResize() && (
                        <div
                          onMouseDown={header.getResizeHandler()}
                          onTouchStart={header.getResizeHandler()}
                          onDoubleClick={() => header.column.resetSize()}
                          className={cn(
                            'absolute right-0 top-0 h-full w-1 cursor-col-resize select-none touch-none',
                            'opacity-0 hover:opacity-100 group-hover/header:opacity-100',
                            header.column.getIsResizing()
                              ? 'bg-primary opacity-100'
                              : 'bg-border',
                          )}
                        />
                      )}
                    </div>
                  </TableHead>
                )
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {isLoading && !table.getRowModel().rows?.length
            ? <TableSkeleton columns={columns.length} densityCls={densityCls} />
            : !table.getRowModel().rows?.length
                ? <EmptyState colSpan={columns.length} message={emptyMessage} icon={emptyIcon} />
                : table.getRowModel().rows.map((row, rowIndex) => (
                    <TableRow
                      key={row.id}
                      data-state={row.getIsSelected() && 'selected'}
                      data-row-index={rowIndex}
                      tabIndex={0}
                      onKeyDown={e => handleKeyDown(e, row.original, rowIndex)}
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
                              width: cell.column.getCanResize() ? cell.column.getSize() : undefined,
                            }}
                          >
                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                          </TableCell>
                        )
                      })}
                    </TableRow>
                  ))}
        </TableBody>
        {showFooter && (
          <TableFooter className={stickyHeader ? 'sticky bottom-0 z-10' : undefined}>
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
