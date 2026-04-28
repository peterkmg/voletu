import { describe, expect, it, vi } from 'vitest'
import {
  buildDateTree,
  commitFilter,
  getSelectedSet,
  selectAllState,
} from '~/components/data-table/column-filter-state'
import { detectFilterType } from '~/components/data-table/filter-utils'

describe('detectFilterType', () => {
  it('classifies numeric facet values as number filters', () => {
    const facets = new Map<unknown, number>([[42, 3], [100, 5]])

    expect(detectFilterType(facets)).toBe('number')
  })

  it('classifies ISO date facet values as date filters', () => {
    const facets = new Map<unknown, number>([['2026-03-31T18:50:22', 2], ['2026-03-30', 1]])

    expect(detectFilterType(facets)).toBe('date')
  })

  it('classifies plain string facet values as text filters', () => {
    const facets = new Map<unknown, number>([['DRAFT', 10], ['EXECUTED', 5]])

    expect(detectFilterType(facets)).toBe('text')
  })

  it('falls back to text filters when no facet values exist', () => {
    expect(detectFilterType(new Map<unknown, number>())).toBe('text')
  })

  it('ignores null facet values when choosing the filter type', () => {
    const facets = new Map<unknown, number>([[null, 1], ['hello', 3]])

    expect(detectFilterType(facets)).toBe('text')
  })
})

describe('buildDateTree', () => {
  it('normalizes datetime facets to day values and aggregates duplicate days', () => {
    const facets = new Map<unknown, number>([
      ['2026-03-31T18:50:22', 2],
      ['2026-03-31T08:00:00', 3],
      ['2026-03-30', 1],
    ])

    const tree = buildDateTree(facets)

    expect(tree.allDates).toEqual(['2026-03-31', '2026-03-30'])
    expect(tree.years[0]?.months[0]?.days).toEqual([
      { date: '2026-03-31', count: 5 },
      { date: '2026-03-30', count: 1 },
    ])
  })

  it('sorts years, months, and days from newest to oldest', () => {
    const facets = new Map<unknown, number>([
      ['2025-12-01', 1],
      ['2026-01-01', 1],
      ['2026-03-01', 1],
    ])

    const tree = buildDateTree(facets)

    expect(tree.years.map(year => year.year)).toEqual([2026, 2025])
    expect(tree.years[0]?.months.map(month => month.month)).toEqual([3, 1])
  })
})

describe('getSelectedSet', () => {
  it('treats a missing filter value as all values selected', () => {
    expect([...getSelectedSet(undefined, ['A', 'B'])]).toEqual(['A', 'B'])
  })

  it('keeps an explicit empty filter value as nothing selected', () => {
    expect([...getSelectedSet([], ['A', 'B'])]).toEqual([])
  })

  it('keeps a partial filter value as the selected subset', () => {
    expect([...getSelectedSet(['A', 'C'], ['A', 'B', 'C'])]).toEqual(['A', 'C'])
  })
})

describe('commitFilter', () => {
  it('clears the column filter when every available value is selected', () => {
    const column = { setFilterValue: vi.fn() }

    commitFilter(column, new Set(['A', 'B']), ['A', 'B'])

    expect(column.setFilterValue).toHaveBeenCalledWith(undefined)
  })

  it('commits the selected subset when only some values are selected', () => {
    const column = { setFilterValue: vi.fn() }

    commitFilter(column, new Set(['A']), ['A', 'B'])

    expect(column.setFilterValue).toHaveBeenCalledWith(['A'])
  })
})

describe('selectAllState', () => {
  it('returns true when every value is selected', () => {
    expect(selectAllState(['A', 'B'], new Set(['A', 'B']))).toBe(true)
  })

  it('returns false when no value is selected', () => {
    expect(selectAllState(['A', 'B'], new Set())).toBe(false)
  })

  it('returns indeterminate when some values are selected', () => {
    expect(selectAllState(['A', 'B', 'C'], new Set(['A']))).toBe('indeterminate')
  })

  it('returns false when there are no available values', () => {
    expect(selectAllState([], new Set(['A']))).toBe(false)
  })
})
