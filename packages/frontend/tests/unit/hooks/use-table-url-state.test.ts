import { act, renderHook } from '@testing-library/react'
import { useTableUrlState } from '~/hooks/use-table-url-state'

function setup(
  search: Record<string, unknown> = {},
  overrides: Record<string, unknown> = {},
) {
  const navigate = vi.fn()
  const result = renderHook(() =>
    useTableUrlState({ search, navigate, ...overrides }),
  )
  return { ...result, navigate }
}

describe('useTableUrlState', () => {
  describe('pagination', () => {
    it('returns default pagination when search is empty', () => {
      const { result } = setup()
      expect(result.current.pagination).toEqual({ pageIndex: 0, pageSize: 10 })
    })

    it('reads page and pageSize from search params', () => {
      const { result } = setup({ page: 3, pageSize: 25 })
      expect(result.current.pagination).toEqual({ pageIndex: 2, pageSize: 25 })
    })

    it('onPaginationChange navigates with updated search', () => {
      const { result, navigate } = setup()

      act(() => {
        result.current.onPaginationChange({ pageIndex: 2, pageSize: 10 })
      })

      expect(navigate).toHaveBeenCalledWith(
        expect.objectContaining({ search: expect.any(Function) }),
      )

      // Extract and call the search updater function
      const searchFn = navigate.mock.calls[0]![0].search
      const updated = searchFn({})
      expect(updated.page).toBe(3) // pageIndex 2 → page 3
    })

    it('omits default page value from search', () => {
      const { result, navigate } = setup({ page: 3 })

      act(() => {
        result.current.onPaginationChange({ pageIndex: 0, pageSize: 10 })
      })

      const searchFn = navigate.mock.calls[0]![0].search
      const updated = searchFn({})
      expect(updated.page).toBeUndefined() // page 1 is default, omitted
    })
  })

  describe('global filter', () => {
    it('returns empty string as default global filter', () => {
      const { result } = setup()
      expect(result.current.globalFilter).toBe('')
    })

    it('reads global filter from search params', () => {
      const { result } = setup({ filter: 'hello' })
      expect(result.current.globalFilter).toBe('hello')
    })

    it('onGlobalFilterChange navigates and resets page', () => {
      const { result, navigate } = setup()

      act(() => {
        result.current.onGlobalFilterChange!('search-term')
      })

      const searchFn = navigate.mock.calls[0]![0].search
      const updated = searchFn({ page: 3 })
      expect(updated.filter).toBe('search-term')
      expect(updated.page).toBeUndefined() // reset to page 1
    })

    it('disables global filter when enabled=false', () => {
      const { result } = setup({}, { globalFilter: { enabled: false } })
      expect(result.current.globalFilter).toBeUndefined()
      expect(result.current.onGlobalFilterChange).toBeUndefined()
    })

    it('resyncs global filter when external search changes', () => {
      const navigate = vi.fn()
      const { result, rerender } = renderHook(
        ({ search }) => useTableUrlState({ search, navigate }),
        { initialProps: { search: { filter: 'hello' } as Record<string, unknown> } },
      )

      expect(result.current.globalFilter).toBe('hello')

      rerender({ search: { filter: 'updated' } })

      expect(result.current.globalFilter).toBe('updated')
    })
  })

  describe('column filters', () => {
    const columnFilterConfig = [
      {
        columnId: 'status',
        searchKey: 'status',
        type: 'string' as const,
      },
      {
        columnId: 'baseIds',
        searchKey: 'bases',
        type: 'array' as const,
        deserialize: (value: unknown) =>
          typeof value === 'string' ? value.split(',').filter(Boolean) : value,
      },
    ]

    it('reads column filters from search params', () => {
      const { result } = setup(
        { status: 'posted', bases: 'a,b' },
        { columnFilters: columnFilterConfig },
      )

      expect(result.current.columnFilters).toEqual([
        { id: 'status', value: 'posted' },
        { id: 'baseIds', value: ['a', 'b'] },
      ])
    })

    it('resyncs column filters when external search changes', () => {
      const navigate = vi.fn()
      const { result, rerender } = renderHook(
        ({ search }) =>
          useTableUrlState({
            search,
            navigate,
            columnFilters: columnFilterConfig,
          }),
        {
          initialProps: {
            search: { status: 'draft', bases: 'x' } as Record<string, unknown>,
          },
        },
      )

      expect(result.current.columnFilters).toEqual([
        { id: 'status', value: 'draft' },
        { id: 'baseIds', value: ['x'] },
      ])

      rerender({ search: { status: 'posted', bases: 'y,z' } })

      expect(result.current.columnFilters).toEqual([
        { id: 'status', value: 'posted' },
        { id: 'baseIds', value: ['y', 'z'] },
      ])
    })
  })

  describe('ensurePageInRange', () => {
    it('navigates to first page if current page exceeds count', () => {
      const { result, navigate } = setup({ page: 10 })

      act(() => {
        result.current.ensurePageInRange(5)
      })

      expect(navigate).toHaveBeenCalledWith(
        expect.objectContaining({ replace: true }),
      )
    })

    it('does nothing when page is within range', () => {
      const { result, navigate } = setup({ page: 3 })

      act(() => {
        result.current.ensurePageInRange(5)
      })

      expect(navigate).not.toHaveBeenCalled()
    })
  })
})
