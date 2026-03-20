import type { SortingState, VisibilityState } from '@tanstack/react-table'
import type { BulkAction } from '~/components/data-table'
import type { TruckWaybillResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import {
  getCoreRowModel,
  getFacetedRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,

  useReactTable,

} from '@tanstack/react-table'
import { Archive } from 'lucide-react'
import { useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { BulkActionsBar, DataTable, DataTablePagination, DataTableToolbar } from '~/components/data-table'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useTableUrlState } from '~/hooks/use-table-url-state'
import { cn } from '~/lib/utils'
import { getTruckWaybillColumns } from './truck-waybills-columns'

const route = getRouteApi('/_authenticated/transport/truck-waybills/')

interface TruckWaybillsTableProps {
  data: TruckWaybillResponse[]
}

export function TruckWaybillsTable({ data }: TruckWaybillsTableProps) {
  const { t } = useTranslation(['transport', 'common'])

  const { data: companiesData } = useCatalogCompanyList()
  const companyMap = useMemo(() => {
    const map = new Map<string, string>()
    for (const c of companiesData?.data ?? []) map.set(c.id, c.commonName)
    return map
  }, [companiesData])

  const columns = useMemo(() => getTruckWaybillColumns(t, { companyMap }), [t, companyMap])

  const [rowSelection, setRowSelection] = useState({})
  const [sorting, setSorting] = useState<SortingState>([])
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({})

  const {
    globalFilter,
    onGlobalFilterChange,
    columnFilters,
    onColumnFiltersChange,
    pagination,
    onPaginationChange,
    ensurePageInRange,
  } = useTableUrlState({
    search: route.useSearch(),
    navigate: route.useNavigate(),
    pagination: { defaultPage: 1, defaultPageSize: 10 },
    globalFilter: { enabled: true, key: 'filter' },
  })

  const table = useReactTable({
    data,
    columns,
    state: {
      sorting,
      columnVisibility,
      rowSelection,
      columnFilters,
      globalFilter,
      pagination,
    },
    enableRowSelection: true,
    onRowSelectionChange: setRowSelection,
    onSortingChange: setSorting,
    onColumnVisibilityChange: setColumnVisibility,
    globalFilterFn: (row, _columnId, filterValue) => {
      const docNumber = String(row.getValue('documentNumber')).toLowerCase()
      const search = String(filterValue).toLowerCase()
      return docNumber.includes(search)
    },
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFacetedRowModel: getFacetedRowModel(),
    getFacetedUniqueValues: getFacetedUniqueValues(),
    onPaginationChange,
    onGlobalFilterChange,
    onColumnFiltersChange,
  })

  const bulkActions: BulkAction<TruckWaybillResponse>[] = [
    {
      label: t('common:actions.softDelete'),
      icon: Archive,
      variant: 'destructive',
      onClick: (rows) => {
        void rows // TODO: wire bulk soft-delete API
      },
    },
  ]

  const pageCount = table.getPageCount()
  useEffect(() => {
    // eslint-disable-next-line react-hooks-extra/no-direct-set-state-in-use-effect
    ensurePageInRange(pageCount)
  }, [pageCount, ensurePageInRange])

  return (
    <div className={cn('flex flex-1 flex-col gap-4')}>
      <DataTableToolbar
        table={table}
        searchPlaceholder={`${t('common:actions.search')}...`}
        filters={[]}
      />
      <DataTable table={table} columns={columns} />
      <DataTablePagination table={table} className="mt-auto" />
      <BulkActionsBar table={table} actions={bulkActions} />
    </div>
  )
}
