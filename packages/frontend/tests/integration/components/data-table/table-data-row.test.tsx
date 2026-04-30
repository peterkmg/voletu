import type { Cell, Column, ColumnDef, Row } from '@tanstack/react-table'
import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithProviders } from '@tests/common'
import { DataRow } from '~/components/data-table/table-data-row'

vi.mock('@tanstack/react-table', async () => {
  const actual = await vi.importActual('@tanstack/react-table')
  return {
    ...actual,
    flexRender: (component: any, context: any) => {
      if (typeof component === 'function')
        return component(context)
      return component
    },
  }
})

interface TestData {
  id: string
  name: string
}

function createMockCell(overrides: Partial<Cell<TestData, unknown>> = {}) {
  return {
    id: 'cell-1',
    column: {
      columnDef: {
        meta: undefined,
        cell: () => 'Cell Content',
      } as unknown as ColumnDef<TestData>,
      getIsPinned: () => false,
    } as unknown as Column<TestData>,
    getContext: () => ({}) as any,
    ...overrides,
  } as unknown as Cell<TestData, unknown>
}

function createMockRow(overrides: Partial<Row<TestData>> = {}) {
  return {
    id: 'row-1',
    original: { id: '1', name: 'Test Item' },
    getIsSelected: () => false,
    getVisibleCells: () => [createMockCell()],
    ...overrides,
  } as unknown as Row<TestData>
}

const defaultProps = {
  row: createMockRow(),
  rowIndex: 0,
  isPinning: false,
  densityCls: 'py-2',
  onKeyDown: vi.fn(),
}

function renderDataRow(props: Partial<typeof defaultProps> & { onRowAction?: (row: TestData) => void, virtualStart?: number } = {}) {
  return renderWithProviders(
    <div style={{ '--col-template': '1fr' } as React.CSSProperties}>
      <DataRow<TestData> {...defaultProps} {...props} />
    </div>,
  )
}

describe('dataRow', () => {
  it('renders cell content', () => {
    renderDataRow()
    expect(screen.getByText('Cell Content')).toBeInTheDocument()
  })

  it('sets data-row-index attribute', () => {
    renderDataRow({ rowIndex: 5 })
    const row = screen.getByRole('row')
    expect(row).toHaveAttribute('data-row-index', '5')
  })

  it('calls onKeyDown with row data on keyboard event', async () => {
    const onKeyDown = vi.fn()
    renderDataRow({ onKeyDown })

    const row = screen.getByRole('row')
    await userEvent.type(row, '{Enter}')

    expect(onKeyDown).toHaveBeenCalled()
    const [_event, data, index] = onKeyDown.mock.calls[0]!
    expect(data).toEqual({ id: '1', name: 'Test Item' })
    expect(index).toBe(0)
  })

  it('calls onRowAction on double-click', async () => {
    const onRowAction = vi.fn()
    renderDataRow({ onRowAction })

    const row = screen.getByRole('row')
    await userEvent.dblClick(row)

    expect(onRowAction).toHaveBeenCalledWith({ id: '1', name: 'Test Item' })
  })

  it('does not error when onRowAction is not provided', async () => {
    renderDataRow()

    const row = screen.getByRole('row')
    await userEvent.dblClick(row)

    expect(row).toBeInTheDocument()
  })

  it('applies virtualStart as translateY when provided', () => {
    renderDataRow({ virtualStart: 150 })
    const row = screen.getByRole('row')
    expect(row.style.transform).toBe('translateY(150px)')
    expect(row.style.position).toBe('absolute')
  })

  it('does not apply positioning when virtualStart is not provided', () => {
    renderDataRow()
    const row = screen.getByRole('row')
    expect(row.style.transform).toBe('')
    expect(row.style.position).toBe('')
  })
})
