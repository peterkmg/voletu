import type { HeaderGroup } from '@tanstack/react-table'
import { flexRender } from '@tanstack/react-table'
import { TableCell, TableRow } from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { alignClasses } from './table-utils'

interface TableFooterRowProps<TData> {
  footerGroup: HeaderGroup<TData>
}

export function TableFooterRow<TData>({
  footerGroup,
}: TableFooterRowProps<TData>) {
  return (
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
  )
}
