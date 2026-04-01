import type { HeaderGroup } from '@tanstack/react-table'
import { flexRender } from '@tanstack/react-table'
import { TableHead, TableRow } from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { alignClasses, getPinningStyles } from './table-utils'

interface TableHeaderRowProps<TData> {
  headerGroup: HeaderGroup<TData>
  isPinning: boolean
}

export function TableHeaderRow<TData>({
  headerGroup,
  isPinning,
}: TableHeaderRowProps<TData>) {
  return (
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
            style={pinStyle}
          >
            {header.isPlaceholder
              ? null
              : flexRender(header.column.columnDef.header, header.getContext())}
          </TableHead>
        )
      })}
    </TableRow>
  )
}
