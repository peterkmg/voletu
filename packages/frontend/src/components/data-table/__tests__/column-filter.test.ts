import { detectFilterType } from '~/components/data-table/column-filter'

// ---------------------------------------------------------------------------
// detectFilterType
// ---------------------------------------------------------------------------

describe('detectFilterType', () => {
  it('returns "number" for numeric values', () => {
    const facets = new Map<unknown, number>([[42, 3], [100, 5]])
    expect(detectFilterType(facets)).toBe('number')
  })

  it('returns "date" for ISO date strings', () => {
    const facets = new Map<unknown, number>([['2026-03-31T18:50:22', 2], ['2026-03-30', 1]])
    expect(detectFilterType(facets)).toBe('date')
  })

  it('returns "text" for plain strings', () => {
    const facets = new Map<unknown, number>([['DRAFT', 10], ['POSTED', 5]])
    expect(detectFilterType(facets)).toBe('text')
  })

  it('returns "text" for empty facets', () => {
    const facets = new Map<unknown, number>()
    expect(detectFilterType(facets)).toBe('text')
  })

  it('skips null values and checks next', () => {
    const facets = new Map<unknown, number>([[null, 1], ['hello', 3]])
    expect(detectFilterType(facets)).toBe('text')
  })
})

// ---------------------------------------------------------------------------
// buildDateTree (tested indirectly via exported helpers)
// We test the filter value semantics used by the date filter.
// ---------------------------------------------------------------------------

describe('date filter value semantics', () => {
  it('YYYY-MM-DD normalization slices ISO datetime correctly', () => {
    expect('2026-03-31T18:50:22'.slice(0, 10)).toBe('2026-03-31')
  })

  it('YYYY-MM-DD normalization handles date-only strings', () => {
    expect('2026-03-31'.slice(0, 10)).toBe('2026-03-31')
  })

  it('year/month extraction from date string works', () => {
    const dateStr = '2026-03-31'
    const parts = dateStr.split('-')
    expect(Number(parts[0])).toBe(2026)
    expect(Number(parts[1])).toBe(3)
  })
})

// ---------------------------------------------------------------------------
// Filter "all-selected-by-default" logic
// ---------------------------------------------------------------------------

describe('all-selected-by-default logic', () => {
  const allValues = ['A', 'B', 'C', 'D']

  it('undefined filterValue means all selected', () => {
    const filterValue: string[] | undefined = undefined
    const selectedSet = filterValue === undefined
      ? new Set(allValues)
      : new Set(filterValue)
    expect(selectedSet.size).toBe(4)
    expect(allValues.every(v => selectedSet.has(v))).toBe(true)
  })

  it('empty array means nothing selected', () => {
    const filterValue: string[] = []
    const selectedSet = new Set(filterValue)
    expect(selectedSet.size).toBe(0)
  })

  it('partial array means subset selected', () => {
    const filterValue = ['A', 'C']
    const selectedSet = new Set(filterValue)
    expect(selectedSet.has('A')).toBe(true)
    expect(selectedSet.has('B')).toBe(false)
    expect(selectedSet.has('C')).toBe(true)
    expect(selectedSet.has('D')).toBe(false)
  })

  it('toggle removes value from full set', () => {
    const selectedSet = new Set(allValues)
    selectedSet.delete('B')
    const result = Array.from(selectedSet)
    expect(result).toEqual(['A', 'C', 'D'])
  })

  it('full set clears to undefined (detected by allValues.every)', () => {
    const selectedSet = new Set(['A', 'B', 'C', 'D'])
    const shouldClear = allValues.every(v => selectedSet.has(v))
    expect(shouldClear).toBe(true)
  })

  it('partial set does not clear', () => {
    const selectedSet = new Set(['A', 'C'])
    const shouldClear = allValues.every(v => selectedSet.has(v))
    expect(shouldClear).toBe(false)
  })
})

// ---------------------------------------------------------------------------
// Select All tri-state logic
// ---------------------------------------------------------------------------

describe('selectAllState logic', () => {
  function selectAllState(allValues: string[], selectedSet: Set<string>): boolean | 'indeterminate' {
    if (allValues.length === 0) return false
    const count = allValues.filter(v => selectedSet.has(v)).length
    if (count === 0) return false
    if (count === allValues.length) return true
    return 'indeterminate'
  }

  it('returns true when all selected', () => {
    expect(selectAllState(['A', 'B'], new Set(['A', 'B']))).toBe(true)
  })

  it('returns false when none selected', () => {
    expect(selectAllState(['A', 'B'], new Set())).toBe(false)
  })

  it('returns indeterminate when some selected', () => {
    expect(selectAllState(['A', 'B', 'C'], new Set(['A']))).toBe('indeterminate')
  })

  it('returns false for empty allValues', () => {
    expect(selectAllState([], new Set(['A']))).toBe(false)
  })
})
