import type { Row } from '@tanstack/react-table'
import type { CSSProperties } from 'react'
import type { GroupInfo } from './table-utils'
import { flexRender } from '@tanstack/react-table'
import { memo, useCallback, useMemo } from 'react'
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
  /** Group boundary info for visual merge in grouped tables */
  groupInfo?: GroupInfo
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
  groupInfo,
}: DataRowProps<TData>) {
  const style = useMemo(
    () => virtualStart != null ? virtualStyle(virtualStart) : undefined,
    [virtualStart],
  )

  const isContinuation = groupInfo && !groupInfo.isFirstOfGroup

  // Group hover: highlight all rows in the same group
  const handleMouseEnter = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (!groupInfo)
      return
    const container = e.currentTarget.parentElement
    if (!container)
      return
    container.querySelectorAll<HTMLElement>(`[data-group-id="${groupInfo.groupId}"]`)
      .forEach(el => el.setAttribute('data-group-hover', ''))
  }, [groupInfo])

  const handleMouseLeave = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (!groupInfo)
      return
    const container = e.currentTarget.parentElement
    if (!container)
      return
    container.querySelectorAll<HTMLElement>('[data-group-hover]')
      .forEach(el => el.removeAttribute('data-group-hover'))
  }, [groupInfo])

  // Group border styling
  const groupCls = groupInfo
    ? cn(
        // No border between rows within a group
        !groupInfo.isLastOfGroup && 'border-b-transparent',
        // Suppress default hover on grouped rows (group hover handles it)
        'hover:bg-transparent data-[group-hover]:bg-muted/50',
      )
    : ''

  return (
    <TableRow
      data-state={row.getIsSelected() && 'selected'}
      data-virtual-index={rowIndex}
      data-row-index={rowIndex}
      data-index={rowIndex}
      data-group-id={groupInfo?.groupId}
      ref={measureRef}
      tabIndex={0}
      onKeyDown={e => onKeyDown(e, row.original, rowIndex)}
      onDoubleClick={onRowAction ? () => onRowAction(row.original) : undefined}
      onMouseEnter={groupInfo ? handleMouseEnter : undefined}
      onMouseLeave={groupInfo ? handleMouseLeave : undefined}
      className={cn('focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring', groupCls)}
      style={style}
    >
      {row.getVisibleCells().map((cell) => {
        const meta = cell.column.columnDef.meta
        const alignCls = meta?.align ? alignClasses[meta.align] : ''
        const pinStyle = isPinning ? getPinningStyles(cell.column) : {}
        const pinBg = cell.column.getIsPinned() ? 'bg-background' : ''

        // Visual merge: suppress doc-level cell content on continuation rows
        const suppressContent = isContinuation && meta?.groupRole === 'doc'

        return (
          <TableCell
            key={cell.id}
            className={cn(meta?.className, meta?.tdClassName, alignCls, densityCls, pinBg)}
            style={pinStyle}
          >
            {suppressContent ? null : flexRender(cell.column.columnDef.cell, cell.getContext())}
          </TableCell>
        )
      })}
    </TableRow>
  )
}

export const DataRow = memo(DataRowInner) as typeof DataRowInner
