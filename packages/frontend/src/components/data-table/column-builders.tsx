import type { ColumnDef, Row } from '@tanstack/react-table'
import { Checkbox } from '~/components/ui/checkbox'
import { DateCell, NumericCell, ResolvedCell } from './cell-renderers'
import { DataTableColumnHeader } from './column-header'
import { StatusBadge } from './status-badge'

export function selectColumn<T>(): ColumnDef<T> {
  return {
    id: 'select',
    header: ({ table }) => (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected()
          || (table.getIsSomePageRowsSelected() && 'indeterminate')
        }
        onCheckedChange={value => table.toggleAllPageRowsSelected(!!value)}
        aria-label="Select all"
        className="translate-y-[2px]"
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={value => row.toggleSelected(!!value)}
        aria-label="Select row"
        className="translate-y-[2px]"
      />
    ),
    enableSorting: false,
    enableHiding: false,
  }
}

export function actionsColumn<T>(
  Actions: React.ComponentType<{ row: Row<T> }>,
): ColumnDef<T> {
  return {
    id: 'actions',
    cell: ({ row }) => <Actions row={row} />,
  }
}

export function dateColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  opts?: { align?: 'left' | 'right', className?: string },
): ColumnDef<T> {
  return {
    accessorKey,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: {
      align: opts?.align ?? 'right' as const,
      ...(opts?.className ? { className: opts.className } : {}),
    },
    cell: ({ row }) => <DateCell value={row.getValue(accessorKey)} />,
  }
}

export function textColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  opts?: { primary?: boolean, className?: string },
): ColumnDef<T> {
  const primary = opts?.primary ?? true
  return {
    accessorKey,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    ...(opts?.className ? { meta: { className: opts.className } } : {}),
    cell: ({ row }) =>
      primary
        ? (
            <span className="font-medium">{row.getValue(accessorKey)}</span>
          )
        : (
            <span className="text-muted-foreground">
              {(row.getValue(accessorKey) as string) ?? '\u2014'}
            </span>
          ),
  }
}

export function resolvedColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  resolvedKey: string,
): ColumnDef<T> {
  return {
    accessorKey,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    cell: ({ row }) => (
      <ResolvedCell value={(row.original as Record<string, unknown>)[resolvedKey] as string} />
    ),
  }
}

export function statusColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  colorMap: Record<string, string>,
): ColumnDef<T> {
  return {
    accessorKey,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    cell: ({ row }) => (
      <StatusBadge value={row.getValue(accessorKey)} colorMap={colorMap} />
    ),
  }
}

export function numericColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  opts?: { align?: 'left' | 'right', padWidth?: number },
): ColumnDef<T> {
  return {
    accessorKey,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: { align: opts?.align ?? 'right' as const },
    cell: ({ row }) => (
      <NumericCell value={row.getValue(accessorKey)} padWidth={opts?.padWidth} />
    ),
  }
}
