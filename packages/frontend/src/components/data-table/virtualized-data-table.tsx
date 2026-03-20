import type { Column, ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import type { CSSProperties } from 'react'
import { flexRender } from '@tanstack/react-table'
import { useVirtualizer } from '@tanstack/react-virtual'
import { useCallback, useRef } from 'react'
import { useTranslation } from 'react-i18next'
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
  /** Called when a row receives Enter key or is double-clicked. */
  onRowAction?: (row: TData) => void
}

const alignClasses = {
  left: 'text-left',
  center: 'text-center',
  right: 'text-right',
}

function getPinningStyles<TData>(column: Column<TData, unknown>): CSSProperties {
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

function hasAnyPinning<TData>(table: TanstackTable<TData>): boolean {
  return table.getLeftLeafColumns().length > 0 || table.getRightLeafColumns().length > 0
}

function hasAnyFooter<TData>(table: TanstackTable<TData>): boolean {
  return table.getFooterGroups().some(group =>
    group.headers.some(header => header.column.columnDef.footer),
  )
}

export function VirtualizedDataTable<TData>({
  table,
  columns,
  className,
  height = '600px',
  estimateSize = 40,
  overscan = 20,
  onRowAction,
}: VirtualizedDataTableProps<TData>) {
  const { t } = useTranslation('common')
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

  // Keyboard navigation
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTableRowElement>, rowData: TData, virtualIndex: number) => {
      switch (e.key) {
        case 'ArrowDown': {
          e.preventDefault()
          const next = containerRef.current?.querySelector<HTMLElement>(
            `[data-virtual-index="${virtualIndex + 1}"]`,
          )
          if (next)
            next.focus()
          else virtualizer.scrollToIndex(virtualIndex + 1)
          break
        }
        case 'ArrowUp': {
          e.preventDefault()
          const prev = containerRef.current?.querySelector<HTMLElement>(
            `[data-virtual-index="${virtualIndex - 1}"]`,
          )
          if (prev)
            prev.focus()
          else virtualizer.scrollToIndex(Math.max(0, virtualIndex - 1))
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
    [rows, onRowAction, virtualizer],
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
                const pinBg = header.column.getIsPinned() ? 'bg-background' : ''
                return (
                  <TableHead
                    key={header.id}
                    colSpan={header.colSpan}
                    className={cn(meta?.className, meta?.thClassName, alignCls, pinBg)}
                    style={{
                      ...pinStyle,
                      width: header.column.getCanResize() ? header.getSize() : undefined,
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
            ? (
                <TableRow>
                  <TableCell colSpan={columns.length} className="h-24 text-center">
                    {t('table.noResults')}
                  </TableCell>
                </TableRow>
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
                                width: cell.column.getCanResize() ? cell.column.getSize() : undefined,
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
