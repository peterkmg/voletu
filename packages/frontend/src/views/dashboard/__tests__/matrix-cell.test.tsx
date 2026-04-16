// packages/frontend/src/views/dashboard/__tests__/matrix-cell.test.tsx
import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { MatrixCell } from '../components/matrix-cell'

function wrapTable(ui: React.ReactNode) {
  return render(<table><tbody><tr>{ui}</tr></tbody></table>)
}

describe('matrixCell', () => {
  it('renders the em dash for undefined amount', () => {
    wrapTable(<MatrixCell productId="p" storageId="s" amount={undefined} />)
    expect(screen.getByRole('cell')).toHaveTextContent('\u2014')
  })

  it('renders the em dash for zero', () => {
    wrapTable(<MatrixCell productId="p" storageId="s" amount={0} />)
    expect(screen.getByRole('cell')).toHaveTextContent('\u2014')
  })

  it('renders a button with formatted amount for a non-zero value', () => {
    wrapTable(<MatrixCell productId="p" storageId="s" amount={123.456} />)
    // French locale formatting: "123,456" with 3 decimals
    expect(screen.getByRole('button')).toHaveTextContent('123,456')
  })

  it('calls onClick when the cell is activated', () => {
    const onClick = vi.fn()
    wrapTable(<MatrixCell productId="p" storageId="s" amount={5} onClick={onClick} />)
    fireEvent.click(screen.getByRole('button'))
    expect(onClick).toHaveBeenCalledOnce()
  })
})
