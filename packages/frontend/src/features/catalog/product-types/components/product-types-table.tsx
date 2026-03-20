import type { SortingState, VisibilityState } from '@tanstack/react-table'
import type { BulkAction } from '~/components/data-table'
import type { ProductTypeResponse } from '~/generated/types'
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
import { useTableUrlState } from '~/hooks/use-table-url-state'
import { cn } from '~/lib/utils'
import { getProductTypeColumns } from './product-types-columns'

const route = getRouteApi('/_authenticated/catalog/product-types/')

interface ProductTypesTableProps {
  data: ProductTypeResponse[]
}

export function ProductTypesTable({ data }: ProductTypesTableProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const columns = useMemo(() => getProductTypeColumns(t), [t])

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
      const longName = String(row.original.longName ?? '').toLowerCase()
      const search = String(filterValue).toLowerCase()
      return name.includes(search) || longName.includes(search)
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

  const bulkActions: BulkAction<ProductTypeResponse>[] = [
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
