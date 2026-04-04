import type { ColumnDef, Row } from '@tanstack/react-table'
import { Checkbox } from '~/components/ui/checkbox'
import { StatusBadge } from '~/components/ui/status-badge'
import { DateCell, NumericCell, ResolvedCell } from './cell-renderers'
import { DataTableColumnHeader } from './column-header'

export function selectColumn<T>(): ColumnDef<T> {
  return {
    id: 'select',
    size: 36,
    minSize: 36,
    maxSize: 36,
    enableResizing: false,
    enableSorting: false,
    enableHiding: false,
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
  }
}

export function actionsColumn<T>(
  Actions: React.ComponentType<{ row: Row<T> }>,
): ColumnDef<T> {
  return {
    id: 'actions',
    size: 72,
    minSize: 72,
    maxSize: 72,
    enableResizing: false,
    enableHiding: false,
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
      label: title,
      align: opts?.align ?? 'left' as const,
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
    meta: {
      label: title,
      ...(opts?.className ? { className: opts.className } : {}),
    },
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
    meta: { label: title },
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
    meta: { label: title },
    cell: ({ row }) => (
      <StatusBadge value={row.getValue(accessorKey)} colorMap={colorMap} />
    ),
  }
}

export function numericColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  opts?: { align?: 'left' | 'right', padWidth?: number, unit?: string },
): ColumnDef<T> {
  return {
    accessorKey,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: { label: title, align: opts?.align ?? 'right' as const },
    cell: ({ row }) => (
      <NumericCell value={row.getValue(accessorKey)} padWidth={opts?.padWidth} unit={opts?.unit} />
    ),
  }
}
