import type { Column, Table as TanstackTable } from '@tanstack/react-table'
import {
  alignClasses,
  getGridTemplate,
  getPinningStyles,
  hasAnyFooter,
  hasAnyPinning,
} from '~/components/data-table/table-utils'

// ---------------------------------------------------------------------------
// Helpers to build mock Column / Table objects
// ---------------------------------------------------------------------------

function mockColumn(overrides: {
  id?: string
  isPinned?: false | 'left' | 'right'
  start?: number
  after?: number
  size?: number
  minSize?: number | undefined
  maxSize?: number | undefined
  meta?: { sizingCategory?: 'fixed' | 'capped' | 'flex' }
} = {}): Column<unknown, unknown> {
  const {
    id = 'col',
    isPinned = false,
    start = 0,
    after = 0,
    size = 150,
    minSize,
    maxSize,
    meta,
  } = overrides

  return {
    id,
    getIsPinned: vi.fn(() => isPinned),
    getStart: vi.fn(() => start),
    getAfter: vi.fn(() => after),
    getSize: vi.fn(() => size),
    columnDef: { minSize, maxSize, meta },
  } as unknown as Column<unknown, unknown>
}

function mockTable(overrides: {
  leftLeafColumns?: Column<unknown, unknown>[]
  rightLeafColumns?: Column<unknown, unknown>[]
  footerGroups?: { headers: { column: { columnDef: { footer?: unknown } } }[] }[]
  visibleLeafColumns?: Column<unknown, unknown>[]
  columnSizing?: Record<string, number>
} = {}): TanstackTable<unknown> {
  const {
    leftLeafColumns = [],
    rightLeafColumns = [],
    footerGroups = [],
    visibleLeafColumns = [],
    columnSizing = {},
  } = overrides

  return {
    getLeftLeafColumns: vi.fn(() => leftLeafColumns),
    getRightLeafColumns: vi.fn(() => rightLeafColumns),
    getFooterGroups: vi.fn(() => footerGroups),
    getVisibleLeafColumns: vi.fn(() => visibleLeafColumns),
    getState: vi.fn(() => ({ columnSizing })),
  } as unknown as TanstackTable<unknown>
}

// ---------------------------------------------------------------------------
// alignClasses (smoke check)
// ---------------------------------------------------------------------------

describe('alignClasses', () => {
  it('maps left/center/right to the correct tailwind classes', () => {
    expect(alignClasses.left).toBe('text-left justify-start')
    expect(alignClasses.center).toBe('text-center justify-center')
    expect(alignClasses.right).toBe('text-right justify-end')
  })
})

// ---------------------------------------------------------------------------
// getPinningStyles
// ---------------------------------------------------------------------------

describe('getPinningStyles', () => {
  it('returns an empty object for an unpinned column', () => {
    const col = mockColumn({ isPinned: false })
    expect(getPinningStyles(col)).toEqual({})
  })

  it('returns sticky + left for a left-pinned column', () => {
    const col = mockColumn({ isPinned: 'left', start: 40 })
    const styles = getPinningStyles(col)

    expect(styles.position).toBe('sticky')
    expect(styles.left).toBe('40px')
    expect(styles.right).toBeUndefined()
    expect(styles.zIndex).toBe(1)
  })

  it('returns sticky + right for a right-pinned column', () => {
    const col = mockColumn({ isPinned: 'right', after: 60 })
    const styles = getPinningStyles(col)

    expect(styles.position).toBe('sticky')
    expect(styles.right).toBe('60px')
    expect(styles.left).toBeUndefined()
    expect(styles.zIndex).toBe(1)
  })
})

// ---------------------------------------------------------------------------
// hasAnyPinning
// ---------------------------------------------------------------------------

describe('hasAnyPinning', () => {
  it('returns false when no columns are pinned', () => {
    const table = mockTable({ leftLeafColumns: [], rightLeafColumns: [] })
    expect(hasAnyPinning(table)).toBe(false)
  })

  it('returns true when left-pinned columns exist', () => {
    const table = mockTable({
      leftLeafColumns: [mockColumn()],
      rightLeafColumns: [],
    })
    expect(hasAnyPinning(table)).toBe(true)
  })

  it('returns true when right-pinned columns exist', () => {
    const table = mockTable({
      leftLeafColumns: [],
      rightLeafColumns: [mockColumn()],
    })
    expect(hasAnyPinning(table)).toBe(true)
  })
})

// ---------------------------------------------------------------------------
// hasAnyFooter
// ---------------------------------------------------------------------------

describe('hasAnyFooter', () => {
  it('returns false when no footers are defined', () => {
    const table = mockTable({
      footerGroups: [
        { headers: [{ column: { columnDef: { footer: undefined } } }] },
      ],
    })
    expect(hasAnyFooter(table)).toBe(false)
  })

  it('returns true when a footer is defined', () => {
    const table = mockTable({
      footerGroups: [
        {
          headers: [
            { column: { columnDef: { footer: undefined } } },
            { column: { columnDef: { footer: 'Total' } } },
          ],
        },
      ],
    })
    expect(hasAnyFooter(table)).toBe(true)
  })
})

// ---------------------------------------------------------------------------
// getGridTemplate
// ---------------------------------------------------------------------------

describe('getGridTemplate', () => {
  it('returns px for fixed-width columns (minSize === maxSize)', () => {
    const col = mockColumn({ id: 'a', size: 100, minSize: 100, maxSize: 100 })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('100px')
  })

  it('returns minmax for flexible columns (no maxSize)', () => {
    const col = mockColumn({ id: 'a', size: 200, minSize: 120, maxSize: undefined })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('minmax(120px, 1fr)')
  })

  it('handles mixed fixed + flexible columns', () => {
    const fixed = mockColumn({ id: 'fixed', size: 50, minSize: 50, maxSize: 50 })
    const flex = mockColumn({ id: 'flex', size: 200, minSize: 100, maxSize: undefined })
    const table = mockTable({
      visibleLeafColumns: [fixed, flex],
      columnSizing: {},
    })

    expect(getGridTemplate(table)).toBe('50px minmax(100px, 1fr)')
  })

  it('uses getSize() for manually resized columns (sizing[col.id] exists)', () => {
    const col = mockColumn({ id: 'resized', size: 250, minSize: 80, maxSize: undefined })
    const table = mockTable({
      visibleLeafColumns: [col],
      columnSizing: { resized: 250 },
    })

    expect(getGridTemplate(table)).toBe('250px')
  })

  it('defaults minSize to 80 when not specified', () => {
    const col = mockColumn({ id: 'a', size: 150, minSize: undefined, maxSize: undefined })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('minmax(80px, 1fr)')
  })

  // --- sizingCategory tests ---

  it('returns exact px for sizingCategory=fixed', () => {
    const col = mockColumn({ id: 'a', size: 36, minSize: 36, maxSize: 36, meta: { sizingCategory: 'fixed' } })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('36px')
  })

  it('returns minmax with px max for sizingCategory=capped', () => {
    const col = mockColumn({ id: 'a', size: 150, minSize: 100, maxSize: 130, meta: { sizingCategory: 'capped' } })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('minmax(100px, 130px)')
  })

  it('defaults capped max to 150 when maxSize is undefined', () => {
    const col = mockColumn({ id: 'a', size: 150, minSize: 90, maxSize: undefined, meta: { sizingCategory: 'capped' } })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('minmax(90px, 150px)')
  })

  it('returns minmax with 1fr for sizingCategory=flex', () => {
    const col = mockColumn({ id: 'a', size: 200, minSize: 120, maxSize: undefined, meta: { sizingCategory: 'flex' } })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('minmax(120px, 1fr)')
  })

  it('defaults flex min to 120 when minSize is undefined', () => {
    const col = mockColumn({ id: 'a', size: 200, minSize: undefined, maxSize: undefined, meta: { sizingCategory: 'flex' } })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('minmax(120px, 1fr)')
  })

  it('manual resize takes priority over sizingCategory', () => {
    const col = mockColumn({ id: 'resized', size: 300, minSize: 100, maxSize: 130, meta: { sizingCategory: 'capped' } })
    const table = mockTable({ visibleLeafColumns: [col], columnSizing: { resized: 300 } })

    expect(getGridTemplate(table)).toBe('300px')
  })

  it('handles mixed sizing categories correctly', () => {
    const fixed = mockColumn({ id: 'sel', size: 36, minSize: 36, maxSize: 36, meta: { sizingCategory: 'fixed' } })
    const capped = mockColumn({ id: 'date', size: 130, minSize: 100, maxSize: 130, meta: { sizingCategory: 'capped' } })
    const flex = mockColumn({ id: 'name', size: 200, minSize: 120, meta: { sizingCategory: 'flex' } })
    const table = mockTable({ visibleLeafColumns: [fixed, capped, flex], columnSizing: {} })

    expect(getGridTemplate(table)).toBe('36px minmax(100px, 130px) minmax(120px, 1fr)')
  })
})
