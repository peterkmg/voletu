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
})

describe('actionsColumn', () => {
  const DummyActions = (() => null) as React.ComponentType<{ row: Row<TestRow> }>
  const col = actionsColumn<TestRow>(DummyActions)

  it('returns ColumnDef with id "actions"', () => {
    expect(col.id).toBe('actions')
  })

  it('has fixed size 72 (size, minSize, maxSize all 72)', () => {
    expect(col.size).toBe(72)
    expect(col.minSize).toBe(72)
    expect(col.maxSize).toBe(72)
  })

  it('disables resizing and hiding', () => {
    expect(col.enableResizing).toBe(false)
    expect(col.enableHiding).toBe(false)
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
})

describe('numericColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = numericColumn<TestRow>('amount', 'Amount')
    expect(col).toHaveProperty('accessorKey', 'amount')
    expect(col.meta).toMatchObject({ label: 'Amount' })
  })

  it('defaults align to "left"', () => {
    const col = numericColumn<TestRow>('amount', 'Amount')
    expect(col.meta).toMatchObject({ align: 'left' })
  })
})

describe('statusColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = statusColumn<TestRow>('status', 'Status', { active: 'green', inactive: 'red' })
    expect(col).toHaveProperty('accessorKey', 'status')
    expect(col.meta).toMatchObject({ label: 'Status' })
  })
})

describe('resolvedColumn', () => {
  it('sets accessorKey and meta.label', () => {
    const col = resolvedColumn<TestRow>('name', 'Owner', 'resolvedName')
    expect(col).toHaveProperty('accessorKey', 'name')
    expect(col.meta).toMatchObject({ label: 'Owner' })
  })
})
