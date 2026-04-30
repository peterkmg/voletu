import type { ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import { useCallback, useState } from 'react'
import {
  Table,
  TableFooter,
  TableHeader,
} from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { densityClasses, useDensity } from './density'
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
  height?: string
  estimateSize?: number
  overscan?: number
  isLoading?: boolean
  emptyMessage?: string
  emptyIcon?: React.ReactNode
  onRowAction?: (row: TData) => void
  groupKey?: string
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
  groupKey,
}: DataTableProps<TData>) {
  const { density } = useDensity()
  const densityCls = densityClasses[density]

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
                groupKey={groupKey}
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
                groupKey={groupKey}
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
