import type { SortingState, VisibilityState } from '@tanstack/react-table'
import type { ProductResponse } from '~/generated/types'
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
import { useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { DataTable, DataTablePagination, DataTableToolbar } from '~/components/data-table'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { useTableUrlState } from '~/hooks/use-table-url-state'
import { cn } from '~/lib/utils'
import { getProductColumns } from './products-columns'

const route = getRouteApi('/_authenticated/catalog/products/')

interface ProductsTableProps {
  data: ProductResponse[]
}

export function ProductsTable({ data }: ProductsTableProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const { data: productGroupsData } = useCatalogProductGroupList()
  const productGroupMap = useMemo(() => {
    const map = new Map<string, string>()
    for (const pg of productGroupsData?.data ?? []) map.set(pg.id, pg.commonName)
    return map
  }, [productGroupsData])

  const { data: companiesData } = useCatalogCompanyList()
  const companyMap = useMemo(() => {
    const map = new Map<string, string>()
    for (const c of companiesData?.data ?? []) map.set(c.id, c.commonName)
    return map
  }, [companiesData])

  const columns = useMemo(() => getProductColumns(t, { productGroupMap, companyMap }), [t, productGroupMap, companyMap])

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
      const name = String(row.getValue('commonName')).toLowerCase()
      const identification = String(row.original.addIdentification ?? '').toLowerCase()
      const search = String(filterValue).toLowerCase()
      return name.includes(search) || identification.includes(search)
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
    </div>
  )
}
