import type { Column, ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import type { CSSProperties } from 'react'
import { flexRender } from '@tanstack/react-table'
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

interface DataTableProps<TData> {
  table: TanstackTable<TData>
  columns: ColumnDef<TData, unknown>[]
  className?: string
  /** Enable sticky column headers when scrolling vertically. Default: true */
  stickyHeader?: boolean
  /** Max height for scrollable table container. Enables vertical scrolling. */
  maxHeight?: string
  /** Called when a row receives Enter key or is double-clicked. */
  onRowAction?: (row: TData) => void
}

const alignClasses = {
  left: 'text-left',
  center: 'text-center',
  right: 'text-right',
}

/** Compute sticky positioning styles for pinned columns */
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

export function DataTable<TData>({
  table,
  columns,
  className,
  stickyHeader = true,
  maxHeight,
  onRowAction,
}: DataTableProps<TData>) {
  const { t } = useTranslation('common')
  const { density } = useDensity()
  const densityCls = densityClasses[density]
  const tableRef = useRef<HTMLDivElement>(null)

  const isPinning = hasAnyPinning(table)
  const showFooter = hasAnyFooter(table)
  const isResizing = table.getState().columnSizingInfo.isResizingColumn

  // Keyboard navigation handler
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTableRowElement>, rowData: TData, rowIndex: number) => {
      const rows = table.getRowModel().rows
      switch (e.key) {
        case 'ArrowDown': {
          e.preventDefault()
          const next = tableRef.current?.querySelector<HTMLElement>(
            `[data-row-index="${rowIndex + 1}"]`,
          )
          next?.focus()
          break
        }
        case 'ArrowUp': {
          e.preventDefault()
          const prev = tableRef.current?.querySelector<HTMLElement>(
            `[data-row-index="${rowIndex - 1}"]`,
          )
          prev?.focus()
          break
        }
        case 'Home': {
          e.preventDefault()
          tableRef.current?.querySelector<HTMLElement>('[data-row-index="0"]')?.focus()
          break
        }
        case 'End': {
          e.preventDefault()
          tableRef.current
            ?.querySelector<HTMLElement>(`[data-row-index="${rows.length - 1}"]`)
            ?.focus()
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
    [table, onRowAction],
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
          {table.getRowModel().rows?.length
            ? table.getRowModel().rows.map((row, rowIndex) => (
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
              ))
            : (
                <TableRow>
                  <TableCell colSpan={columns.length} className="h-24 text-center">
                    {t('table.noResults')}
                  </TableCell>
                </TableRow>
              )}
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
