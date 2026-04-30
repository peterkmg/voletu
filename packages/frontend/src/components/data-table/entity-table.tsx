import type {
  ColumnDef,
  Row,
  SortingState,
  VisibilityState,
} from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BulkAction } from './bulk-actions-bar'
import type { TableMode } from './table-mode-toggle'
import {
  getCoreRowModel,
  getFacetedRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from '@tanstack/react-table'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useTableUrlState } from '~/hooks/use-table-url-state'
import { BulkActionsBar } from './bulk-actions-bar'
import { DataTable } from './data-table'
import { DataTablePagination } from './pagination'
import { getStoredTableMode } from './table-mode-storage'
import { DataTableToolbar } from './toolbar'

interface EntityTableProps<T> {
  data: T[]
  getColumns: (t: TFunction) => ColumnDef<T>[]
  routeApi: {
    useSearch: () => Record<string, unknown>
    useNavigate: () => any
  }
  globalFilterFn: (
    row: Row<T>,
    columnId: string,
    filterValue: string,
  ) => boolean
  i18nNamespaces: string[]
  isLoading?: boolean
  bulkActions?: (t: TFunction) => BulkAction<T>[]
  /** Unique ID for persisting table preferences (e.g. 'companies'). */
  tableId?: string
  /** Field name for row grouping (visual merge). When set, doc-level cells suppress on continuation rows. */
  groupKey?: keyof T & string
  /** Optional action buttons rendered in the toolbar (e.g. Create button). */
  actions?: React.ReactNode
  /** Base filename for table exports. Defaults to `tableId` when available. */
  exportFilename?: string
  /** Force a specific table mode and hide the mode toggle. */
  forcedTableMode?: TableMode
  /** Use server-provided pagination instead of slicing locally. */
  serverPagination?: {
    pageCount: number
  }
  /** Keep filter state in the URL while applying it on the server. */
  manualFiltering?: boolean
  /** Optional stable row id for TanStack table rows. */
  getRowId?: (row: T, index: number) => string
  /** Enable row selection behavior. */
  enableRowSelection?: boolean
  /** Default page size for URL-backed pagination. */
  defaultPageSize?: number
}

export function EntityTable<T>({
  data,
  getColumns,
  routeApi,
  globalFilterFn,
  i18nNamespaces,
  isLoading,
  bulkActions,
  tableId,
  groupKey,
  actions,
  exportFilename,
  forcedTableMode,
  serverPagination,
  manualFiltering = false,
  getRowId,
  enableRowSelection = true,
  defaultPageSize = 10,
}: EntityTableProps<T>) {
  const { t } = useTranslation(i18nNamespaces)
  const columns = useMemo(() => getColumns(t), [t, getColumns])

  const [storedTableMode, setStoredTableMode] = useState<TableMode>(
    () => forcedTableMode ?? getStoredTableMode(tableId),
  )
  const [rowSelection, setRowSelection] = useState({})
  const [sorting, setSorting] = useState<SortingState>([])
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>(
    () => {
      const defaults: VisibilityState = {}
      for (const col of columns) {
        if (col.meta?.requiresRole) {
          const key = (col as { accessorKey?: string }).accessorKey ?? col.id
          if (key)
            defaults[key] = false
        }
      }
      return defaults
    },
  )
  const tableMode = forcedTableMode ?? storedTableMode

  const handleModeChange = useCallback(
    (mode: TableMode) => {
      if (forcedTableMode)
        return
      setStoredTableMode(mode)
      if (tableId) {
        try {
          localStorage.setItem(`table-mode-${tableId}`, mode)
        }
        catch {
          /* ignore */
        }
      }
    },
    [forcedTableMode, tableId],
  )

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
    pagination: {
      defaultPage: 1,
      defaultPageSize: tableMode === 'virtual' ? 9999 : defaultPageSize,
    },
    globalFilter: { enabled: true, key: 'filter' },
  })

  const table = useReactTable({
    data,
    columns,
    getRowId,
    state: {
      sorting,
      columnVisibility,
      rowSelection,
      columnFilters,
      globalFilter,
      pagination,
    },
    enableRowSelection,
    onRowSelectionChange: setRowSelection,
    onSortingChange: setSorting,
    onColumnVisibilityChange: setColumnVisibility,
    globalFilterFn,
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: manualFiltering ? undefined : getFilteredRowModel(),
    getPaginationRowModel: serverPagination
      ? undefined
      : getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFacetedRowModel: getFacetedRowModel(),
    getFacetedUniqueValues: getFacetedUniqueValues(),
    onPaginationChange,
    onGlobalFilterChange,
    onColumnFiltersChange,
    manualFiltering,
    manualPagination: !!serverPagination,
    pageCount: serverPagination?.pageCount,
  })

  const resolvedBulkActions = useMemo(
    () => bulkActions?.(t) ?? [],
    [t, bulkActions],
  )

  const pageCount = serverPagination?.pageCount ?? table.getPageCount()
  useEffect(() => {
    ensurePageInRange(pageCount)
  }, [pageCount, ensurePageInRange])

  return (
    <div className="flex flex-1 flex-col gap-4 min-h-0">
      <DataTableToolbar
        table={table}
        searchPlaceholder={`${t('common:actions.search')}...`}
        tableMode={forcedTableMode ? undefined : tableMode}
        onTableModeChange={forcedTableMode ? undefined : handleModeChange}
        exportFilename={exportFilename ?? tableId}
        actions={actions}
      />
      <div className="flex-1 min-h-0">
        <DataTable
          table={table}
          columns={columns}
          mode={tableMode}
          isLoading={isLoading}
          height="100%"
          groupKey={groupKey}
        />
      </div>
      {tableMode === 'paginated' && (
        <DataTablePagination table={table} className="mt-auto" />
      )}
      {resolvedBulkActions.length > 0 && (
        <BulkActionsBar table={table} actions={resolvedBulkActions} />
      )}
    </div>
  )
}
