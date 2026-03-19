import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { CompanyResponse } from '~/generated/types'
import { DataTableColumnHeader } from '~/components/data-table'
import { Badge } from '~/components/ui/badge'
import { Checkbox } from '~/components/ui/checkbox'
import { DataTableRowActions } from './data-table-row-actions'

export function getCompanyColumns(t: TFunction): ColumnDef<CompanyResponse>[] {
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
      accessorKey: 'commonName',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:company.columns.commonName')}
        />
      ),
      meta: { className: 'w-1/3' },
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('commonName')}</span>
      ),
    },
    {
      accessorKey: 'legalName',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('catalog:company.columns.legalName')}
        />
      ),
      meta: { className: 'w-1/4' },
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('legalName') ?? '—'}
        </span>
      ),
    },
    {
      id: 'roles',
      header: t('common:table.status'),
      cell: ({ row }) => {
        const flags = [
          { key: 'isContractor', label: t('catalog:company.columns.isContractor') },
          { key: 'isExporter', label: t('catalog:company.columns.isExporter') },
          { key: 'isManufacturer', label: t('catalog:company.columns.isManufacturer') },
          { key: 'isSender', label: t('catalog:company.columns.isSender') },
        ] as const

        const active = flags.filter(
          f => row.original[f.key as keyof typeof row.original],
        )

        return (
          <div className="flex flex-wrap gap-1">
            {active.map(f => (
              <Badge key={f.key} variant="outline" className="text-xs">
                {f.label}
              </Badge>
            ))}
          </div>
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
