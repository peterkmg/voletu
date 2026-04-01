import type { ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import { useCallback, useState } from 'react'
import {
  Table,
  TableFooter,
  TableHeader,
} from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { densityClasses, useDensity } from './density-context'
import { PaginatedTableBody } from './paginated-table-body'
import { TableFooterRow } from './table-footer-row'
import { TableHeaderRow } from './table-header-row'
import { getGridTemplate, hasAnyFooter, hasAnyPinning } from './table-utils'
import { VirtualTableBody } from './virtual-table-body'

interface DataTableProps<TData> {
  table: TanstackTable<TData>
  columns: ColumnDef<TData, unknown>[]
  mode?: 'virtual' | 'paginated'
  className?: string
  /** Height of the scrollable container (virtual mode). Default: '600px' */
  height?: string
  /** Estimated row height in pixels (virtual mode). Default: 40 */
  estimateSize?: number
  /** Number of rows to render outside visible area (virtual mode). Default: 20 */
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

export function DataTable<TData>({
  table,
  columns,
  mode = 'virtual',
  className,
  height = '600px',
  estimateSize = 40,
  overscan = 20,
  isLoading,
  emptyMessage,
  emptyIcon,
  onRowAction,
}: DataTableProps<TData>) {
  const { density } = useDensity()
  const densityCls = densityClasses[density]
  // useState (not useRef) so that when the div mounts, the state change
  // triggers a re-render and the virtualizer picks up the scroll element.
  const [scrollEl, setScrollEl] = useState<HTMLDivElement | null>(null)
  const scrollRefCb = useCallback((node: HTMLDivElement | null) => setScrollEl(node), [])

  const gridTemplate = getGridTemplate(table)
  const isPinning = hasAnyPinning(table)
  const showFooter = hasAnyFooter(table)

  return (
    <div
      ref={scrollRefCb}
      className={cn('overflow-auto rounded-md border', className)}
      style={{ maxHeight: height }}
    >
      <Table gridTemplate={gridTemplate}>
        <TableHeader className="sticky top-0 z-10 bg-background">
          {table.getHeaderGroups().map(hg => (
            <TableHeaderRow key={hg.id} headerGroup={hg} isPinning={isPinning} />
          ))}
        </TableHeader>
        {mode === 'virtual'
          ? (
              <VirtualTableBody
                table={table}
                columns={columns}
                scrollElement={scrollEl}
                isPinning={isPinning}
                densityCls={densityCls}
                overscan={overscan}
                estimateSize={estimateSize}
                isLoading={isLoading}
                emptyMessage={emptyMessage}
                emptyIcon={emptyIcon}
                onRowAction={onRowAction}
              />
            )
          : (
              <PaginatedTableBody
                table={table}
                columns={columns}
                isPinning={isPinning}
                densityCls={densityCls}
                isLoading={isLoading}
                emptyMessage={emptyMessage}
                emptyIcon={emptyIcon}
                onRowAction={onRowAction}
              />
            )}
        {showFooter && (
          <TableFooter className="sticky bottom-0 z-10">
            {table.getFooterGroups().map(fg => (
              <TableFooterRow key={fg.id} footerGroup={fg} />
            ))}
          </TableFooter>
        )}
      </Table>
    </div>
  )
}
