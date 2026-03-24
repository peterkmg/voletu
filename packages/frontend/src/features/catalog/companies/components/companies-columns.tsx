import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { CompanyResponse } from '~/generated/types'
import { actionsColumn, dateColumn, selectColumn, StatusBadge, textColumn } from '~/components/data-table'
import { companyRoleColors } from '~/lib/badge-colors'
import { DataTableRowActions } from './data-table-row-actions'

export function getCompanyColumns(t: TFunction): ColumnDef<CompanyResponse>[] {
  return [
    selectColumn<CompanyResponse>(),
    textColumn<CompanyResponse>('commonName', t('catalog:company.columns.commonName'), { className: 'w-1/3' }),
    textColumn<CompanyResponse>('legalName', t('catalog:company.columns.legalName'), { primary: false, className: 'w-1/4' }),
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
              <StatusBadge key={f.key} value={f.key} label={f.label} colorMap={companyRoleColors} className="text-xs" />
            ))}
          </div>
        )
      },
    },
    dateColumn<CompanyResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<CompanyResponse>(DataTableRowActions),
  ]
}
