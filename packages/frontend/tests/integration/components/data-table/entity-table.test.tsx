import type { ColumnDef } from '@tanstack/react-table'
import { useReactTable } from '@tanstack/react-table'
import { act, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithProviders } from '@tests/common'
import { textColumn } from '~/components/data-table'
import { setTableDensityPreference } from '~/components/data-table/density-state'
import { EntityTable } from '~/components/data-table/entity-table'
import { TooltipProvider } from '~/components/ui/tooltip'

const reactTableMock = vi.hoisted(() => ({
  options: [] as unknown[],
}))

const tableUrlStateMock = vi.hoisted(() => ({
  calls: [] as unknown[],
}))

const fileSaveMock = vi.hoisted(() => ({
  saveExportFile: vi.fn().mockResolvedValue({ status: 'saved', target: 'browser' }),
}))

vi.mock('@tanstack/react-table', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@tanstack/react-table')>()
  return {
    ...actual,
    useReactTable: vi.fn((options: Parameters<typeof actual.useReactTable>[0]) => {
      reactTableMock.options.push(options)
      return actual.useReactTable(options)
    }),
  }
})

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('~/hooks/use-table-url-state', () => ({
  useTableUrlState: vi.fn((config) => {
    tableUrlStateMock.calls.push(config)
    return {
      globalFilter: '',
      onGlobalFilterChange: vi.fn(),
      columnFilters: [],
      onColumnFiltersChange: vi.fn(),
      pagination: {
        pageIndex: 0,
        pageSize: config.pagination.defaultPageSize,
      },
      onPaginationChange: vi.fn(),
      ensurePageInRange: vi.fn(),
    }
  }),
}))

vi.mock('~/lib/files', () => ({
  saveExportFile: fileSaveMock.saveExportFile,
}))

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

function renderEntityTable(
  data: SimpleItem[] = [],
  tableId?: string,
  props: Partial<React.ComponentProps<typeof EntityTable<SimpleItem>>> = {},
) {
  return renderWithProviders(
    <TooltipProvider>
      <EntityTable<SimpleItem>
        data={data}
        getColumns={getColumns}
        routeApi={mockRouteApi}
        globalFilterFn={globalFilterFn}
        i18nNamespaces={['common']}
        tableId={tableId}
        {...props}
      />
    </TooltipProvider>,
  )
}

describe('entityTable', () => {
  afterEach(() => {
    localStorage.clear()
    reactTableMock.options = []
    tableUrlStateMock.calls = []
    fileSaveMock.saveExportFile.mockClear()
    vi.mocked(useReactTable).mockClear()
  })

  it('renders toolbar with search input', () => {
    renderEntityTable()

    const input = screen.getByPlaceholderText('common:actions.search...')
    expect(input).toBeInTheDocument()
  })

  it('renders table with data', () => {
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

    expect(screen.queryByRole('button', { name: /last page/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /next page/i })).not.toBeInTheDocument()
  })

  it('updates mounted table density from the shared density source', () => {
    localStorage.setItem('table-mode-test', 'paginated')
    renderEntityTable([{ id: '1', name: 'Alpha' }], 'test')

    expect(screen.getByText('Alpha').parentElement).toHaveClass('py-2')

    act(() => {
      setTableDensityPreference('compact')
    })

    expect(screen.getByText('Alpha').parentElement).toHaveClass('py-1')
  })

  it('passes custom row identity and row selection settings to the table', () => {
    const getRowId = (row: SimpleItem) => `simple:${row.id}`

    renderEntityTable([{ id: '1', name: 'Alpha' }], 'test', {
      enableRowSelection: false,
      getRowId,
    })

    expect(reactTableMock.options[reactTableMock.options.length - 1]).toMatchObject({
      enableRowSelection: false,
      getRowId,
    })
  })

  it('uses a custom default page size for url-backed pagination', () => {
    localStorage.setItem('table-mode-test', 'paginated')

    renderEntityTable([{ id: '1', name: 'Alpha' }], 'test', {
      defaultPageSize: 25,
    })

    expect(tableUrlStateMock.calls[tableUrlStateMock.calls.length - 1]).toMatchObject({
      pagination: {
        defaultPage: 1,
        defaultPageSize: 25,
      },
    })
  })

  it('exports the filtered table using the configured export filename', async () => {
    const user = userEvent.setup()
    localStorage.setItem('table-mode-test', 'paginated')

    renderEntityTable([{ id: '1', name: 'Alpha' }], 'test', {
      exportFilename: 'simple-items',
    })

    await user.click(screen.getByRole('button', { name: 'tables:export' }))

    expect(fileSaveMock.saveExportFile).toHaveBeenCalledTimes(1)
    const savedFile = fileSaveMock.saveExportFile.mock.calls[0]?.[0]
    expect(savedFile).toMatchObject({
      suggestedName: expect.stringMatching(/^simple-items-\d{4}-\d{2}-\d{2}\.csv$/),
      mimeType: 'text/csv;charset=utf-8',
    })
    expect(savedFile?.contents).toContain('Alpha')
  })
})
