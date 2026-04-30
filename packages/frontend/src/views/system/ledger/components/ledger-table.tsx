import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { LedgerBalanceResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { DataTableColumnHeader, EntityTable, numericColumn } from '~/components/data-table'

function getLedgerColumns(t: TFunction): ColumnDef<LedgerBalanceResponse>[] {
  return [
    {
      accessorKey: 'storageId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.storage')}
        />
      ),
      cell: ({ row }) => (
        <span className="font-medium">{row.getValue('storageId')}</span>
      ),
    },
    {
      accessorKey: 'productId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.product')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('productId')}
        </span>
      ),
    },
    {
      accessorKey: 'contractorId',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:ledger.columns.contractor')}
        />
      ),
      cell: ({ row }) => (
        <span className="text-muted-foreground">
          {row.getValue('contractorId')}
        </span>
      ),
    },
    numericColumn<LedgerBalanceResponse>('currentAmount', t('system:ledger.columns.quantity')),
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
  ]
}

const route = getRouteApi('/_authenticated/ledger/')

interface LedgerTableProps {
  data: LedgerBalanceResponse[]
}

function ledgerGlobalFilterFn(
  row: Row<LedgerBalanceResponse>,
  _columnId: string,
  filterValue: string,
) {
  const storage = String(row.getValue('storageId')).toLowerCase()
  const product = String(row.getValue('productId')).toLowerCase()
  const contractor = String(row.getValue('contractorId')).toLowerCase()
  const search = String(filterValue).toLowerCase()
  return storage.includes(search) || product.includes(search) || contractor.includes(search)
}

export function LedgerTable({ data }: LedgerTableProps) {
  return (
    <EntityTable<LedgerBalanceResponse>
      tableId="ledger"
      data={data}
      getColumns={getLedgerColumns}
      routeApi={route}
      globalFilterFn={ledgerGlobalFilterFn}
      i18nNamespaces={['system', 'common']}
      getRowId={row => `${row.storageId}:${row.productId}:${row.contractorId}`}
      enableRowSelection={false}
      forcedTableMode="paginated"
    />
  )
}
