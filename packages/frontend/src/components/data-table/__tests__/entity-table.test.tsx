import type { ColumnDef } from '@tanstack/react-table'
import { screen } from '@testing-library/react'
import { textColumn } from '~/components/data-table'
import { TooltipProvider } from '~/components/ui/tooltip'
import { renderWithProviders } from '~/test-utils'
import { EntityTable } from '../entity-table'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('~/hooks/use-table-url-state', () => ({
  useTableUrlState: () => ({
    globalFilter: '',
    onGlobalFilterChange: vi.fn(),
    columnFilters: [],
    onColumnFiltersChange: vi.fn(),
    pagination: { pageIndex: 0, pageSize: 9999 },
    onPaginationChange: vi.fn(),
    ensurePageInRange: vi.fn(),
  }),
}))

// ---------- test helpers ----------

interface SimpleItem {
  id: string
  name: string
}

function getColumns(_t: any): ColumnDef<SimpleItem, unknown>[] {
  return [
    textColumn<SimpleItem>('name', 'Name'),
  ]
}

const globalFilterFn = () => true

const mockRouteApi = {
  useSearch: () => ({}),
  useNavigate: () => vi.fn(),
}

function renderEntityTable(data: SimpleItem[] = [], tableId?: string) {
  return renderWithProviders(
    <TooltipProvider>
      <EntityTable<SimpleItem>
        data={data}
        getColumns={getColumns}
        routeApi={mockRouteApi}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        tableId={tableId}
      />
    </TooltipProvider>,
  )
}

// ---------- tests ----------

describe('entityTable', () => {
  afterEach(() => {
    localStorage.clear()
  })

  it('renders toolbar with search input', () => {
    renderEntityTable()

    const input = screen.getByPlaceholderText('common:actions.search...')
    expect(input).toBeInTheDocument()
  })

  it('renders table with data', () => {
    // Use paginated mode because the virtualizer requires real DOM dimensions
    // that jsdom cannot provide.
    localStorage.setItem('table-mode-test', 'paginated')
    const data: SimpleItem[] = [
      { id: '1', name: 'Alpha' },
      { id: '2', name: 'Beta' },
    ]

    renderEntityTable(data, 'test')

    expect(screen.getByText('Alpha')).toBeInTheDocument()
    expect(screen.getByText('Beta')).toBeInTheDocument()
  })

  it('renders row content from data', () => {
    localStorage.setItem('table-mode-test', 'paginated')
    const data: SimpleItem[] = [
      { id: '1', name: 'First Item' },
      { id: '2', name: 'Second Item' },
      { id: '3', name: 'Third Item' },
    ]

    renderEntityTable(data, 'test')

    for (const item of data) {
      expect(screen.getByText(item.name)).toBeInTheDocument()
    }
  })

  it('defaults to virtual mode', () => {
    renderEntityTable([{ id: '1', name: 'Solo' }])

    // In virtual mode, no pagination controls are rendered.
    // DataTablePagination renders navigation buttons with specific aria-labels.
    expect(screen.queryByRole('button', { name: /last page/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /next page/i })).not.toBeInTheDocument()
  })
})
