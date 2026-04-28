import type { ColumnDef } from '@tanstack/react-table'
import type { TestItem } from '@tests/common'
import { screen } from '@testing-library/react'
import { createTestData, renderWithProviders, useTestTable } from '@tests/common'
import { selectColumn, textColumn } from '~/components/data-table'
import { DataTable } from '~/components/data-table/data-table'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key === 'tables:noResults' ? 'No results.' : key,
    i18n: { language: 'en' },
  }),
}))

const columns: ColumnDef<TestItem, unknown>[] = [
  selectColumn<TestItem>(),
  textColumn<TestItem>('name', 'Name'),
  textColumn<TestItem>('status', 'Status'),
]

function TestDataTable({
  data,
  mode = 'virtual',
  isLoading,
  height = '400px',
}: {
  data: TestItem[]
  mode?: 'virtual' | 'paginated'
  isLoading?: boolean
  height?: string
}) {
  const table = useTestTable(data, columns)
  return (
    <DataTable
      table={table}
      columns={columns}
      mode={mode}
      height={height}
      isLoading={isLoading}
    />
  )
}

describe('dataTable', () => {
  it('renders table with role="table"', () => {
    renderWithProviders(<TestDataTable data={createTestData(3)} />)
    expect(screen.getByRole('table')).toBeInTheDocument()
  })

  it('renders column headers', () => {
    renderWithProviders(<TestDataTable data={createTestData(3)} />)
    expect(screen.getByRole('columnheader', { name: /name/i })).toBeInTheDocument()
    expect(screen.getByRole('columnheader', { name: /status/i })).toBeInTheDocument()
  })

  it('renders data rows in paginated mode', () => {
    renderWithProviders(<TestDataTable data={createTestData(5)} mode="paginated" />)
    // 1 header row + 5 data rows
    const rows = screen.getAllByRole('row')
    expect(rows.length).toBe(1 + 5)
  })

  it('renders data rows in virtual mode', () => {
    renderWithProviders(<TestDataTable data={createTestData(5)} mode="virtual" />)
    // Virtual body wrapper has position: relative style
    const rowgroups = screen.getAllByRole('rowgroup')
    const body = rowgroups.find(el => el.dataset.slot === 'table-body')
    expect(body).toBeDefined()
    expect(body!.style.position).toBe('relative')
  })

  it('renders empty state when no data', () => {
    renderWithProviders(<TestDataTable data={[]} />)
    expect(screen.getByText('No results.')).toBeInTheDocument()
  })

  it('renders loading skeleton when isLoading with no data', () => {
    renderWithProviders(<TestDataTable data={[]} isLoading />)
    // TableSkeleton renders SKELETON_ROWS (5) rows, each with 3 columns = 15 skeleton cells
    const skeletonCells = screen.getAllByRole('cell')
    expect(skeletonCells.length).toBe(5 * 3)
  })

  it('applies maxHeight style to scroll container', () => {
    renderWithProviders(<TestDataTable data={createTestData(3)} height="300px" />)
    const table = screen.getByRole('table')
    const scrollContainer = table.parentElement!
    expect(scrollContainer.style.maxHeight).toBe('300px')
  })
})
