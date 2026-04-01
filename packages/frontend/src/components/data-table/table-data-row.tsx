import type { Row } from '@tanstack/react-table'
import type { CSSProperties } from 'react'
import { flexRender } from '@tanstack/react-table'
import { memo, useMemo } from 'react'
import { TableCell, TableRow } from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { alignClasses, getPinningStyles } from './table-utils'

interface DataRowProps<TData> {
  row: Row<TData>
  rowIndex: number
  isPinning: boolean
  densityCls: string
  onKeyDown: (e: React.KeyboardEvent<HTMLDivElement>, data: TData, index: number) => void
  onRowAction?: (row: TData) => void
  /** Callback ref for virtualizer measurement (virtual mode only) */
  measureRef?: (el: HTMLElement | null) => void
  /** Virtual row offset — when set, row uses absolute positioning with translateY */
  virtualStart?: number
}

/** Positioning style for virtual rows — only recomputed when virtualStart changes */
function virtualStyle(start: number): CSSProperties {
  return {
    position: 'absolute',
    top: 0,
    width: '100%',
    transform: `translateY(${start}px)`,
  }
}

function DataRowInner<TData>({
  row,
  rowIndex,
  isPinning,
  densityCls,
  onKeyDown,
  onRowAction,
  measureRef,
  virtualStart,
}: DataRowProps<TData>) {
  const style = useMemo(
    () => virtualStart != null ? virtualStyle(virtualStart) : undefined,
    [virtualStart],
  )

  return (
    <TableRow
      data-state={row.getIsSelected() && 'selected'}
      data-virtual-index={rowIndex}
      data-row-index={rowIndex}
      data-index={rowIndex}
      ref={measureRef}
      tabIndex={0}
      onKeyDown={e => onKeyDown(e, row.original, rowIndex)}
      onDoubleClick={onRowAction ? () => onRowAction(row.original) : undefined}
      className="focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      style={style}
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
            style={pinStyle}
          >
            {flexRender(cell.column.columnDef.cell, cell.getContext())}
          </TableCell>
        )
      })}
    </TableRow>
  )
}

export const DataRow = memo(DataRowInner) as typeof DataRowInner
