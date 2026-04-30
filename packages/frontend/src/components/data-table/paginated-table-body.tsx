import type { ColumnDef, Table as TanstackTable } from '@tanstack/react-table'
import { useCallback, useRef } from 'react'
import { TableBody } from '~/components/ui/table'
import { DataRow } from './table-data-row'
import { EmptyState, TableSkeleton } from './table-states'
import { computeGroupInfo } from './table-utils'

interface PaginatedTableBodyProps<TData> {
  table: TanstackTable<TData>
  columns: ColumnDef<TData, unknown>[]
  isPinning: boolean
  densityCls: string
  isLoading?: boolean
  emptyMessage?: string
  emptyIcon?: React.ReactNode
  onRowAction?: (row: TData) => void

  groupKey?: string
}

export function PaginatedTableBody<TData>({
  table,
  columns,
  isPinning,
  densityCls,
  isLoading,
  emptyMessage,
  emptyIcon,
  onRowAction,
  groupKey,
}: PaginatedTableBodyProps<TData>) {
  const bodyRef = useRef<HTMLDivElement>(null)
  const rows = table.getRowModel().rows

  const focusRow = useCallback((el: HTMLElement | null | undefined) => {
    if (!el)
      return
    el.focus({ preventScroll: true })
    el.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
  }, [])

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>, rowData: TData, rowIndex: number) => {
      const allRows = table.getRowModel().rows
      switch (e.key) {
        case 'ArrowDown': {
          e.preventDefault()
          focusRow(bodyRef.current?.querySelector(`[data-row-index="${rowIndex + 1}"]`))
          break
        }
        case 'ArrowUp': {
          e.preventDefault()
          focusRow(bodyRef.current?.querySelector(`[data-row-index="${rowIndex - 1}"]`))
          break
        }
        case 'Home': {
          e.preventDefault()
          focusRow(bodyRef.current?.querySelector('[data-row-index="0"]'))
          break
        }
        case 'End': {
          e.preventDefault()
          focusRow(bodyRef.current?.querySelector(`[data-row-index="${allRows.length - 1}"]`))
          break
        }
        case 'Enter': {
          e.preventDefault()
          onRowAction?.(rowData)
          break
        }
        case ' ': {
          e.preventDefault()
          allRows[rowIndex]?.toggleSelected()
          break
        }
      }
    },
    [table, onRowAction, focusRow],
  )

  return (
    <TableBody ref={bodyRef}>
      {isLoading && !rows.length
        ? <TableSkeleton columns={columns.length} densityCls={densityCls} />
        : !rows.length
            ? <EmptyState colSpan={columns.length} message={emptyMessage} icon={emptyIcon} />
            : rows.map((row, rowIndex) => (
                <DataRow
                  key={row.id}
                  row={row}
                  rowIndex={rowIndex}
                  isPinning={isPinning}
                  densityCls={densityCls}
                  onKeyDown={handleKeyDown}
                  onRowAction={onRowAction}
                  groupInfo={groupKey ? computeGroupInfo(rows, rowIndex, groupKey) : undefined}
                />
              ))}
    </TableBody>
  )
}
