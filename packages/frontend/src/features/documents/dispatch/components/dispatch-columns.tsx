import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getDispatchColumns(t: TFunction): ColumnDef<DispatchResponse>[] {
  return [
    {
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
    },
    {
      accessorKey: 'documentNumber',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:dispatch.columns.documentNumber')}
        />
      ),
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('documentNumber')}</span>
      ),
    },
    {
      accessorKey: 'date',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:dispatch.columns.date')}
        />
      ),
      cell: ({ row }) => {
        const date = row.getValue<string>('date')
        return (
          <span className="text-muted-foreground text-sm">
            {new Date(date).toLocaleDateString()}
          </span>
        )
      },
    },
    {
      accessorKey: 'dispatchPurpose',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:dispatch.columns.purpose')}
        />
      ),
      cell: ({ row }) => (
        <Badge variant="outline">{row.getValue('dispatchPurpose')}</Badge>
      ),
    },
    {
      accessorKey: 'dispatchMethod',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:dispatch.columns.method')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('dispatchMethod')}
        </span>
      ),
    },
    {
      accessorKey: 'status',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('documents:dispatch.columns.status')}
        />
      ),
      cell: ({ row }) => {
        const status = row.getValue<string>('status')
        return (
          <Badge variant={status === 'Draft' ? 'outline' : 'default'}>
            {status === 'Draft'
              ? t('common:status.draft')
              : t('common:status.executed')}
          </Badge>
        )
      },
    },
    {
      accessorKey: 'createdAt',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.createdAt')}
        />
      ),
      cell: ({ row }) => {
        const date = row.getValue<string>('createdAt')
        return (
          <span className="text-muted-foreground text-sm">
            {new Date(date).toLocaleDateString()}
          </span>
        )
      },
    },
    {
      id: 'actions',
      cell: ({ row }) => <DataTableRowActions row={row} />,
    },
  ]
}
