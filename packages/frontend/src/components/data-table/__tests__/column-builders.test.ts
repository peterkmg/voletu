import type { Row } from '@tanstack/react-table'
import {
  actionsColumn,
  dateColumn,
  numericColumn,
  resolvedColumn,
  selectColumn,
  statusColumn,
  textColumn,
} from '~/components/data-table/column-builders'

interface TestRow { name: string, date: string, amount: number, status: string, resolvedName: string }

describe('selectColumn', () => {
  const col = selectColumn<TestRow>()

  it('returns ColumnDef with id "select"', () => {
    expect(col.id).toBe('select')
  })

  it('has fixed size 36 (size, minSize, maxSize all 36)', () => {
    expect(col.size).toBe(36)
    expect(col.minSize).toBe(36)
    expect(col.maxSize).toBe(36)
  })

  it('disables resizing, sorting, and hiding', () => {
    expect(col.enableResizing).toBe(false)
    expect(col.enableSorting).toBe(false)
    expect(col.enableHiding).toBe(false)
  })

  it('has sizingCategory "fixed"', () => {
    expect(col.meta).toMatchObject({ sizingCategory: 'fixed' })
  })
})

describe('actionsColumn', () => {
  const DummyActions = (() => null) as React.ComponentType<{ row: Row<TestRow> }>
  const col = actionsColumn<TestRow>(DummyActions)

  it('returns ColumnDef with id "actions"', () => {
    expect(col.id).toBe('actions')
  })

  it('calculates fixed size from slot count (default 3 slots)', () => {
    // 3 slots * 32px + 16px padding = 112
    const expectedWidth = 3 * 32 + 16
    expect(col.size).toBe(expectedWidth)
    expect(col.minSize).toBe(expectedWidth)
    expect(col.maxSize).toBe(expectedWidth)
  })

  it('disables resizing and hiding', () => {
    expect(col.enableResizing).toBe(false)
    expect(col.enableHiding).toBe(false)
  })

  it('has sizingCategory "fixed"', () => {
    expect(col.meta).toMatchObject({ sizingCategory: 'fixed' })
  })
})

describe('textColumn', () => {
  const col = textColumn<TestRow>('name', 'Name')

  it('sets accessorKey from first argument', () => {
    expect(col).toHaveProperty('accessorKey', 'name')
  })

  it('sets meta.label from title argument', () => {
    expect(col.meta).toMatchObject({ label: 'Name' })
  })

  it('has sizingCategory "flex" by default with minSize 120', () => {
    expect(col.meta).toMatchObject({ sizingCategory: 'flex' })
    expect(col.minSize).toBe(120)
    expect(col.maxSize).toBeUndefined()
  })

  it('supports sizing="capped" override with maxSize', () => {
    const capped = textColumn<TestRow>('name', 'Doc #', { sizing: 'capped', maxSize: 160 })
    expect(capped.meta).toMatchObject({ sizingCategory: 'capped' })
    expect(capped.maxSize).toBe(160)
  })

  it('supports custom minSize', () => {
    const custom = textColumn<TestRow>('name', 'Name', { minSize: 200 })
    expect(custom.minSize).toBe(200)
  })

  it('defines header function', () => {
    expect(typeof col.header).toBe('function')
  })

  it('defines cell function', () => {
    expect(typeof col.cell).toBe('function')
  })
})

describe('dateColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = dateColumn<TestRow>('date', 'Created')
    expect(col).toHaveProperty('accessorKey', 'date')
    expect(col.meta).toMatchObject({ label: 'Created' })
  })

  it('defaults align to "left"', () => {
    const col = dateColumn<TestRow>('date', 'Created')
    expect(col.meta).toMatchObject({ align: 'left' })
  })

  it('respects custom align option', () => {
    const col = dateColumn<TestRow>('date', 'Created', { align: 'right' })
    expect(col.meta).toMatchObject({ align: 'right' })
  })

  it('has sizingCategory "capped" with minSize 100 and maxSize 130', () => {
    const col = dateColumn<TestRow>('date', 'Created')
    expect(col.meta).toMatchObject({ sizingCategory: 'capped' })
    expect(col.minSize).toBe(100)
    expect(col.maxSize).toBe(130)
  })

  it('has a custom filterFn that normalizes ISO datetimes to YYYY-MM-DD', () => {
    const col = dateColumn<TestRow>('date', 'Created')
    expect(typeof col.filterFn).toBe('function')

    const filterFn = col.filterFn as (row: any, columnId: string, filterValue: string[] | undefined) => boolean
    const mockRow = { getValue: () => '2026-03-31T18:50:22' }

    // undefined filter → show all
    expect(filterFn(mockRow, 'date', undefined)).toBe(true)

    // empty array → show nothing
    expect(filterFn(mockRow, 'date', [])).toBe(false)

    // matching date
    expect(filterFn(mockRow, 'date', ['2026-03-31'])).toBe(true)

    // non-matching date
    expect(filterFn(mockRow, 'date', ['2026-03-30'])).toBe(false)

    // null row value
    const nullRow = { getValue: () => null }
    expect(filterFn(nullRow, 'date', ['2026-03-31'])).toBe(false)
  })
})

describe('numericColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = numericColumn<TestRow>('amount', 'Amount')
    expect(col).toHaveProperty('accessorKey', 'amount')
    expect(col.meta).toMatchObject({ label: 'Amount' })
  })

  it('defaults align to "right"', () => {
    const col = numericColumn<TestRow>('amount', 'Amount')
    expect(col.meta).toMatchObject({ align: 'right' })
  })

  it('has sizingCategory "capped" with minSize 90 and maxSize 150', () => {
    const col = numericColumn<TestRow>('amount', 'Amount')
    expect(col.meta).toMatchObject({ sizingCategory: 'capped' })
    expect(col.minSize).toBe(90)
    expect(col.maxSize).toBe(150)
  })
})

describe('statusColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = statusColumn<TestRow>('status', 'Status', { active: 'green', inactive: 'red' })
    expect(col).toHaveProperty('accessorKey', 'status')
    expect(col.meta).toMatchObject({ label: 'Status' })
  })

  it('has sizingCategory "capped" with minSize 90 and maxSize 130', () => {
    const col = statusColumn<TestRow>('status', 'Status', { active: 'green' })
    expect(col.meta).toMatchObject({ sizingCategory: 'capped' })
    expect(col.minSize).toBe(90)
    expect(col.maxSize).toBe(130)
  })
})

describe('resolvedColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = resolvedColumn<TestRow>('name', 'Owner', 'resolvedName')
    expect(col).toHaveProperty('accessorKey', 'name')
    expect(col.meta).toMatchObject({ label: 'Owner' })
  })

  it('has sizingCategory "flex" with minSize 120', () => {
    const col = resolvedColumn<TestRow>('name', 'Owner', 'resolvedName')
    expect(col.meta).toMatchObject({ sizingCategory: 'flex' })
    expect(col.minSize).toBe(120)
  })
})
