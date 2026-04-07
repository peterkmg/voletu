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
    meta: { sizingCategory: 'fixed' },
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

/**
 * @param Actions Row actions component to render in the actions cell
 * @param slots Number of visible button slots (1=details only, 2=edit+more, 3=details+edit+more)
 */
export function actionsColumn<T>(
  Actions: React.ComponentType<{ row: Row<T> }>,
  slots: 1 | 2 | 3 = 3,
): ColumnDef<T> {
  // 28px per button (h-7 w-7) + 4px gap + 16px cell padding
  const width = slots * 32 + 16
  return {
    id: 'actions',
    size: width,
    minSize: width,
    maxSize: width,
    enableResizing: false,
    enableHiding: false,
    meta: { sizingCategory: 'fixed' },
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
    minSize: 100,
    maxSize: 130,
    filterFn: (row, columnId, filterValue: string[] | undefined) => {
      if (filterValue === undefined)
        return true
      if (filterValue.length === 0)
        return false
      const val = row.getValue<string>(columnId)
      if (!val)
        return false
      return filterValue.includes(val.slice(0, 10))
    },
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: {
      label: title,
      sizingCategory: 'capped' as const,
      align: opts?.align ?? 'left' as const,
      ...(opts?.className ? { className: opts.className } : {}),
    },
    cell: ({ row }) => <DateCell value={row.getValue(accessorKey)} />,
  }
}

export function textColumn<T>(
  accessorKey: keyof T & string,
  title: string,
  opts?: {
    primary?: boolean
    className?: string
    /** Override sizing category. Defaults to 'flex'. Use 'capped' for bounded fields like document numbers. */
    sizing?: 'capped' | 'flex'
    minSize?: number
    maxSize?: number
  },
): ColumnDef<T> {
  const primary = opts?.primary ?? true
  const sizing = opts?.sizing ?? 'flex'
  return {
    accessorKey,
    minSize: opts?.minSize ?? 120,
    ...(sizing === 'capped' ? { maxSize: opts?.maxSize ?? 160 } : {}),
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: {
      label: title,
      sizingCategory: sizing,
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
    minSize: 120,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: { label: title, sizingCategory: 'flex' as const },
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
    minSize: 90,
    maxSize: 130,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: { label: title, sizingCategory: 'capped' as const },
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
    minSize: 90,
    maxSize: 150,
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title={title} />
    ),
    meta: { label: title, sizingCategory: 'capped' as const, align: opts?.align ?? 'right' as const },
    cell: ({ row }) => (
      <NumericCell value={row.getValue(accessorKey)} padWidth={opts?.padWidth} unit={opts?.unit} />
    ),
  }
}
