import type { ColumnDef, Row, SortingState, VisibilityState } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BulkAction } from './bulk-actions-bar'
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
import { useTableUrlState } from '~/hooks/use-table-url-state'
import { cn } from '~/lib/utils'
import { BulkActionsBar } from './bulk-actions-bar'
import { DataTable } from './data-table'
import { DataTablePagination } from './pagination'
import { DataTableToolbar } from './toolbar'

interface EntityTableProps<T> {
  data: T[]
  getColumns: (t: TFunction) => ColumnDef<T>[]
  routeApi: { useSearch: () => Record<string, unknown>, useNavigate: () => any }
  globalFilterFn: (row: Row<T>, columnId: string, filterValue: string) => boolean
  i18nNamespaces: string[]
  isLoading?: boolean
  bulkActions?: (t: TFunction) => BulkAction<T>[]
}

export function EntityTable<T>({
  data,
  getColumns,
  routeApi,
  globalFilterFn,
  i18nNamespaces,
  isLoading,
  bulkActions,
}: EntityTableProps<T>) {
  const { t } = useTranslation(i18nNamespaces)
  const columns = useMemo(() => getColumns(t), [t, getColumns])

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
    search: routeApi.useSearch(),
    navigate: routeApi.useNavigate(),
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
    globalFilterFn,
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

  const resolvedBulkActions = useMemo(
    () => bulkActions?.(t) ?? [],
    [t, bulkActions],
  )

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
      <DataTable table={table} columns={columns} isLoading={isLoading} />
      <DataTablePagination table={table} className="mt-auto" />
      {resolvedBulkActions.length > 0 && (
        <BulkActionsBar table={table} actions={resolvedBulkActions} />
      )}
    </div>
  )
}
