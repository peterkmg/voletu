import { render, screen } from '@testing-library/react'
import {
  DateCell,
  DateTimeCell,
  IdCell,
  LookupCell,
  NullCell,
  NumericCell,
  ResolvedCell,
} from '~/components/data-table/cell-renderers'
import { TooltipProvider } from '~/components/ui/tooltip'

describe('nullCell', () => {
  it('renders an em-dash', () => {
    render(<NullCell />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })
})

describe('dateCell', () => {
  it('renders NullCell for null value', () => {
    render(<DateCell value={null} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders NullCell for undefined value', () => {
    render(<DateCell value={undefined} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders a formatted date for a valid ISO string', () => {
    render(<DateCell value="2025-06-15T10:30:00Z" />)

    expect(screen.getByText(/2025/)).toBeInTheDocument()
    expect(screen.getByText(/06/)).toBeInTheDocument()
    expect(screen.getByText(/15/)).toBeInTheDocument()
  })
})

describe('dateTimeCell', () => {
  it('renders NullCell for null value', () => {
    render(<DateTimeCell value={null} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders a formatted datetime for a valid ISO string', () => {
    render(<DateTimeCell value="2025-06-15T10:30:00Z" />)

    expect(screen.getByText(/2025/)).toBeInTheDocument()
  })
})

describe('numericCell', () => {
  it('renders NullCell for null value', () => {
    render(<NumericCell value={null} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders a number formatted with 3 decimal places (fr-FR locale)', () => {
    render(<NumericCell value={42} />)
    expect(screen.getByText('42,000')).toBeInTheDocument()
  })

  it('renders a zero-padded number when padWidth is provided', () => {
    render(<NumericCell value={7} padWidth={3} />)
    expect(screen.getByText('007')).toBeInTheDocument()
  })

  it('renders a string value formatted with 3 decimal places (fr-FR locale)', () => {
    render(<NumericCell value="99" />)
    expect(screen.getByText('99,000')).toBeInTheDocument()
  })
})

describe('idCell', () => {
  it('renders NullCell for null value', () => {
    render(<IdCell value={null} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders a truncated ID for long UUIDs', () => {
    const uuid = '550e8400-e29b-41d4-a716-446655440000'
    render(<TooltipProvider><IdCell value={uuid} /></TooltipProvider>)

    expect(screen.getByText('550e8400…')).toBeInTheDocument()
  })

  it('renders a short ID without truncation', () => {
    render(<TooltipProvider><IdCell value="abcd1234" /></TooltipProvider>)
    expect(screen.getByText('abcd1234')).toBeInTheDocument()
  })
})

describe('resolvedCell', () => {
  it('renders NullCell for null value', () => {
    render(<ResolvedCell value={null} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders the text value', () => {
    render(<ResolvedCell value="Active" />)
    expect(screen.getByText('Active')).toBeInTheDocument()
  })
})

describe('lookupCell', () => {
  it('renders NullCell for null value', () => {
    const map = new Map<string, string>()
    render(<LookupCell value={null} lookupMap={map} />)
    expect(screen.getByText('—')).toBeInTheDocument()
  })

  it('renders resolved text when key exists in lookupMap', () => {
    const map = new Map([['key1', 'Resolved Name']])
    render(<LookupCell value="key1" lookupMap={map} />)
    expect(screen.getByText('Resolved Name')).toBeInTheDocument()
  })

  it('falls back to IdCell when key is not in lookupMap', () => {
    const map = new Map<string, string>()
    const uuid = '550e8400-e29b-41d4-a716-446655440000'
    render(<TooltipProvider><LookupCell value={uuid} lookupMap={map} /></TooltipProvider>)

    expect(screen.getByText('550e8400…')).toBeInTheDocument()
  })
})
