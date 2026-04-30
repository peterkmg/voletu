import type {
  ColumnFiltersState,
  OnChangeFn,
  PaginationState,
} from '@tanstack/react-table'
import { useCallback, useEffect, useMemo, useState } from 'react'

type SearchRecord = Record<string, unknown>

interface NavigateOptions<TSearch extends SearchRecord> {
  replace?: boolean
  search: (prev: TSearch) => TSearch
}

export type NavigateFn<TSearch extends SearchRecord> = (
  opts: NavigateOptions<TSearch>,
) => void

interface UseTableUrlStateParams<TSearch extends SearchRecord> {
  search: TSearch
  navigate: NavigateFn<TSearch>
  pagination?: {
    pageKey?: string
    pageSizeKey?: string
    defaultPage?: number
    defaultPageSize?: number
  }
  globalFilter?: {
    enabled?: boolean
    key?: string
    trim?: boolean
  }
  columnFilters?: Array<
    | {
      columnId: string
      searchKey: string
      type?: 'string'
      serialize?: (value: unknown) => unknown
      deserialize?: (value: unknown) => unknown
    }
    | {
      columnId: string
      searchKey: string
      type: 'array'
      serialize?: (value: unknown) => unknown
      deserialize?: (value: unknown) => unknown
    }
  >
}

interface UseTableUrlStateReturn {
  globalFilter?: string
  onGlobalFilterChange?: OnChangeFn<string>
  columnFilters: ColumnFiltersState
  onColumnFiltersChange: OnChangeFn<ColumnFiltersState>
  pagination: PaginationState
  onPaginationChange: OnChangeFn<PaginationState>
  ensurePageInRange: (
    pageCount: number,
    opts?: { resetTo?: 'first' | 'last' },
  ) => void
}

export function useTableUrlState<TSearch extends SearchRecord>(
  params: UseTableUrlStateParams<TSearch>,
): UseTableUrlStateReturn {
  const {
    search,
    navigate,
    pagination: paginationCfg,
    globalFilter: globalFilterCfg,
    columnFilters: columnFiltersCfg = [],
  } = params

  const pageKey = paginationCfg?.pageKey ?? ('page' as string)
  const pageSizeKey = paginationCfg?.pageSizeKey ?? ('pageSize' as string)
  const defaultPage = paginationCfg?.defaultPage ?? 1
  const defaultPageSize = paginationCfg?.defaultPageSize ?? 10

  const globalFilterKey = globalFilterCfg?.key ?? ('filter' as string)
  const globalFilterEnabled = globalFilterCfg?.enabled ?? true
  const trimGlobal = globalFilterCfg?.trim ?? true

  const urlColumnFilters: ColumnFiltersState = useMemo(() => {
    const collected: ColumnFiltersState = []
    for (const cfg of columnFiltersCfg) {
      const raw = (search as SearchRecord)[cfg.searchKey]
      const deserialize = cfg.deserialize ?? ((v: unknown) => v)
      if (cfg.type === 'string') {
        const value = (deserialize(raw) as string) ?? ''
        if (typeof value === 'string' && value.trim() !== '') {
          collected.push({ id: cfg.columnId, value })
        }
      }
      else {
        const value = (deserialize(raw) as unknown[]) ?? []
        if (Array.isArray(value) && value.length > 0) {
          collected.push({ id: cfg.columnId, value })
        }
      }
    }
    return collected
  }, [columnFiltersCfg, search])

  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>(urlColumnFilters)
  useEffect(() => {
    const configuredIds = new Set(columnFiltersCfg.map(cfg => cfg.columnId))
    setColumnFilters((prev) => {
      const next = [
        ...prev.filter(f => !configuredIds.has(f.id)),
        ...urlColumnFilters,
      ]

      if (next.length === prev.length
        && next.every((f, i) => f.id === prev[i]?.id && f.value === prev[i]?.value)) {
        return prev
      }
      return next
    })
  }, [urlColumnFilters, columnFiltersCfg])

  const pagination: PaginationState = useMemo(() => {
    const rawPage = (search as SearchRecord)[pageKey]
    const rawPageSize = (search as SearchRecord)[pageSizeKey]
    const pageNum = typeof rawPage === 'number' ? rawPage : defaultPage
    const pageSizeNum
      = typeof rawPageSize === 'number' ? rawPageSize : defaultPageSize
    return { pageIndex: Math.max(0, pageNum - 1), pageSize: pageSizeNum }
  }, [search, pageKey, pageSizeKey, defaultPage, defaultPageSize])

  const onPaginationChange: OnChangeFn<PaginationState> = useCallback((updater) => {
    const next = typeof updater === 'function' ? updater(pagination) : updater
    const nextPage = next.pageIndex + 1
    const nextPageSize = next.pageSize
    navigate({
      search: (prev: TSearch) => ({
        ...(prev as SearchRecord),
        [pageKey]: nextPage <= defaultPage ? undefined : nextPage,
        [pageSizeKey]:
          nextPageSize === defaultPageSize ? undefined : nextPageSize,
      }) as TSearch,
    })
  }, [pagination, navigate, pageKey, pageSizeKey, defaultPage, defaultPageSize])

  const globalFilter = useMemo<string | undefined>(() => {
    if (!globalFilterEnabled)
      return undefined
    const raw = (search as SearchRecord)[globalFilterKey]
    return typeof raw === 'string' ? raw : ''
  }, [globalFilterEnabled, search, globalFilterKey])

  const onGlobalFilterChangeImpl: OnChangeFn<string> = useCallback((updater) => {
    const next
      = typeof updater === 'function'
        ? updater(globalFilter ?? '')
        : updater
    const value = trimGlobal ? next.trim() : next
    navigate({
      search: (prev: TSearch) => ({
        ...(prev as SearchRecord),
        [pageKey]: undefined,
        [globalFilterKey]: value || undefined,
      }) as TSearch,
    })
  }, [globalFilter, trimGlobal, navigate, pageKey, globalFilterKey])

  const onGlobalFilterChange: OnChangeFn<string> | undefined
    = globalFilterEnabled ? onGlobalFilterChangeImpl : undefined

  const onColumnFiltersChange: OnChangeFn<ColumnFiltersState> = useCallback((updater) => {
    const next
      = typeof updater === 'function' ? updater(columnFilters) : updater

    setColumnFilters(next)

    if (columnFiltersCfg.length === 0)
      return

    const patch: Record<string, unknown> = {}

    for (const cfg of columnFiltersCfg) {
      const found = next.find(f => f.id === cfg.columnId)
      const serialize = cfg.serialize ?? ((v: unknown) => v)
      if (cfg.type === 'string') {
        const value
          = typeof found?.value === 'string' ? (found.value as string) : ''
        patch[cfg.searchKey]
          = value.trim() !== '' ? serialize(value) : undefined
      }
      else {
        const value = Array.isArray(found?.value)
          ? (found!.value as unknown[])
          : []
        patch[cfg.searchKey] = value.length > 0 ? serialize(value) : undefined
      }
    }

    navigate({
      search: (prev: TSearch) => ({
        ...(prev as SearchRecord),
        [pageKey]: undefined,
        ...patch,
      }) as TSearch,
    })
  }, [columnFilters, columnFiltersCfg, navigate, pageKey])

  const ensurePageInRange = useCallback((
    pageCount: number,
    opts: { resetTo?: 'first' | 'last' } = { resetTo: 'first' },
  ) => {
    const currentPage = (search as SearchRecord)[pageKey]
    const pageNum = typeof currentPage === 'number' ? currentPage : defaultPage
    if (pageCount > 0 && pageNum > pageCount) {
      navigate({
        replace: true,
        search: (prev: TSearch) => ({
          ...(prev as SearchRecord),
          [pageKey]: opts.resetTo === 'last' ? pageCount : undefined,
        }) as TSearch,
      })
    }
  }, [search, pageKey, defaultPage, navigate])

  return {
    globalFilter: globalFilterEnabled ? (globalFilter ?? '') : undefined,
    onGlobalFilterChange,
    columnFilters,
    onColumnFiltersChange,
    pagination,
    onPaginationChange,
    ensurePageInRange,
  }
}
