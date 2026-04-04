import { render, screen } from '@testing-library/react'
import {
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from '../table'

describe('table', () => {
  it('renders with role="table"', () => {
    render(<Table>content</Table>)
    expect(screen.getByRole('table')).toBeInTheDocument()
  })

  it('sets --col-template CSS variable when gridTemplate is provided', () => {
    const { container } = render(<Table gridTemplate="1fr 2fr 1fr">content</Table>)
    const el = container.querySelector<HTMLElement>('[data-slot="table"]')!
    expect(el.style.cssText).toContain('--col-template')
    expect(el.style.getPropertyValue('--col-template')).toBe('1fr 2fr 1fr')
  })

  it('does not set --col-template when gridTemplate is not provided', () => {
    const { container } = render(<Table>content</Table>)
    const el = container.querySelector<HTMLElement>('[data-slot="table"]')!
    expect(el.style.getPropertyValue('--col-template')).toBe('')
  })

  it('forwards className', () => {
    render(<Table className="custom-class">content</Table>)
    expect(screen.getByRole('table')).toHaveClass('custom-class')
  })
})

describe('tableRow', () => {
  it('renders with role="row"', () => {
    render(<TableRow>cell</TableRow>)
    expect(screen.getByRole('row')).toBeInTheDocument()
  })

  it('has gridTemplateColumns style referencing --col-template', () => {
    render(<TableRow>cell</TableRow>)
    const row = screen.getByRole('row')
    expect(row.style.gridTemplateColumns).toBe('var(--col-template)')
  })

  it('merges additional style props', () => {
    render(<TableRow style={{ color: 'red' }}>cell</TableRow>)
    const row = screen.getByRole('row')
    expect(row.style.gridTemplateColumns).toBe('var(--col-template)')
    expect(row.style.color).toBe('red')
  })
})

describe('tableHead', () => {
  it('renders with role="columnheader"', () => {
    render(<TableHead>Header</TableHead>)
    expect(screen.getByRole('columnheader')).toBeInTheDocument()
  })

  it('converts colSpan to gridColumn style', () => {
    render(<TableHead colSpan={3}>Header</TableHead>)
    const el = screen.getByRole('columnheader')
    expect(el.style.gridColumn).toBe('span 3')
  })

  it('does not set gridColumn when colSpan is not provided', () => {
    render(<TableHead>Header</TableHead>)
    const el = screen.getByRole('columnheader')
    expect(el.style.gridColumn).toBe('')
  })
})

describe('tableCell', () => {
  it('renders with role="cell"', () => {
    render(<TableCell>Data</TableCell>)
    expect(screen.getByRole('cell')).toBeInTheDocument()
  })

  it('converts colSpan to gridColumn style', () => {
    render(<TableCell colSpan={2}>Data</TableCell>)
    const el = screen.getByRole('cell')
    expect(el.style.gridColumn).toBe('span 2')
  })

  it('does not set gridColumn when colSpan is not provided', () => {
    render(<TableCell>Data</TableCell>)
    const el = screen.getByRole('cell')
    expect(el.style.gridColumn).toBe('')
  })
})

describe('tableHeader', () => {
  it('renders with role="rowgroup"', () => {
    render(<TableHeader>header content</TableHeader>)
    expect(screen.getByRole('rowgroup')).toBeInTheDocument()
  })

  it('has data-slot="table-header"', () => {
    const { container } = render(<TableHeader>header content</TableHeader>)
    expect(container.querySelector('[data-slot="table-header"]')).toBeInTheDocument()
  })
})

describe('tableBody', () => {
  it('renders with role="rowgroup"', () => {
    render(<TableBody>body content</TableBody>)
    expect(screen.getByRole('rowgroup')).toBeInTheDocument()
  })

  it('has data-slot="table-body"', () => {
    const { container } = render(<TableBody>body content</TableBody>)
    expect(container.querySelector('[data-slot="table-body"]')).toBeInTheDocument()
  })
})

describe('tableFooter', () => {
  it('renders with role="rowgroup"', () => {
    render(<TableFooter>footer content</TableFooter>)
    expect(screen.getByRole('rowgroup')).toBeInTheDocument()
  })

  it('has data-slot="table-footer"', () => {
    const { container } = render(<TableFooter>footer content</TableFooter>)
    expect(container.querySelector('[data-slot="table-footer"]')).toBeInTheDocument()
  })
})
