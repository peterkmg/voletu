import { render, screen } from '@testing-library/react'
import { TableHeaderRow } from '~/components/data-table/table-header-row'

vi.mock('@tanstack/react-table', async () => {
  const actual = await vi.importActual('@tanstack/react-table')
  return {
    ...actual,
    flexRender: (component: any, _ctx: any) =>
      typeof component === 'function' ? component(_ctx) : component,
  }
})

function createMockHeaderGroup() {
  return {
    id: 'header-group-0',
    headers: [
      {
        id: 'name',
        colSpan: 1,
        column: {
          columnDef: { meta: undefined, header: 'Name' },
          getIsPinned: () => false,
        },
        isPlaceholder: false,
        getContext: () => ({}),
      },
      {
        id: 'status',
        colSpan: 1,
        column: {
          columnDef: { meta: { align: 'right' as const }, header: 'Status' },
          getIsPinned: () => false,
        },
        isPlaceholder: false,
        getContext: () => ({}),
      },
    ],
  }
}

function renderHeaderRow(headerGroup = createMockHeaderGroup()) {
  return render(
    <div style={{ '--col-template': '1fr 1fr' } as React.CSSProperties}>
      <TableHeaderRow headerGroup={headerGroup as any} isPinning={false} />
    </div>,
  )
}

describe('tableHeaderRow', () => {
  it('renders column headers', () => {
    renderHeaderRow()
    expect(screen.getByText('Name')).toBeInTheDocument()
    expect(screen.getByText('Status')).toBeInTheDocument()
  })

  it('renders all header cells with role="columnheader"', () => {
    renderHeaderRow()
    const headers = screen.getAllByRole('columnheader')
    expect(headers).toHaveLength(2)
  })

  it('does not render content for placeholder headers', () => {
    const headerGroup = createMockHeaderGroup()
    headerGroup.headers[0]!.isPlaceholder = true
    renderHeaderRow(headerGroup)

    // Only Status header should have rendered content
    expect(screen.queryByText('Name')).not.toBeInTheDocument()
    expect(screen.getByText('Status')).toBeInTheDocument()
    // Both columnheader elements still exist (just one is empty)
    expect(screen.getAllByRole('columnheader')).toHaveLength(2)
  })

  it('applies alignment class from column meta', () => {
    renderHeaderRow()
    const headers = screen.getAllByRole('columnheader')
    // Status header has meta.align = 'right'
    const statusHeader = headers.find(h => h.textContent === 'Status')
    expect(statusHeader).toBeDefined()
    expect(statusHeader!.className).toContain('text-right')

    // Name header has no alignment
    const nameHeader = headers.find(h => h.textContent === 'Name')
    expect(nameHeader).toBeDefined()
    expect(nameHeader!.className).not.toContain('text-right')
    expect(nameHeader!.className).not.toContain('text-center')
  })
})
