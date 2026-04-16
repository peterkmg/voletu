import { formatAmount } from '~/lib/formatters'
// packages/frontend/src/views/dashboard/components/matrix-total-cell.tsx
import { cn } from '~/lib/utils'
import { EDGE_SHADOW, Z } from './sticky'

export type MatrixTotalVariant
  = | 'row-total' // rightmost per-row sum
    | 'col-total' // bottom per-column sum
    | 'grand-total' // bottom-right grand total
    | 'row-subtotal' // full-width subtotal row cell
    | 'col-subtotal' // vertical subtotal column cell
    | 'subtotal-intersect' // where row-subtotal crosses col-subtotal

export interface MatrixTotalCellProps {
  amount: number | undefined
  variant?: MatrixTotalVariant
  sticky?: 'right'
}

const DASH = '\u2014'

export function MatrixTotalCell({ amount, variant = 'row-total', sticky }: MatrixTotalCellProps) {
  const isEmpty = amount == null || amount === 0
  const stickyStyle: React.CSSProperties | undefined = sticky === 'right'
    ? { position: 'sticky', right: 0, zIndex: Z.bodyRight, boxShadow: EDGE_SHADOW.stickyRight }
    : undefined
  return (
    <td
      className={cn(
        'text-right tabular-nums px-[var(--cell-px)] py-[var(--cell-py)] border-b border-r border-border/50 font-medium bg-muted text-[length:var(--font-size)]',
        variant === 'grand-total' && 'font-semibold border-t-2',
        variant === 'row-subtotal' && 'border-t border-b border-border',
        variant === 'col-subtotal' && 'border-l border-r border-border',
        variant === 'subtotal-intersect' && 'font-semibold border-t border-b border-l border-r border-border',
        isEmpty && 'text-muted-foreground',
      )}
      style={stickyStyle}
    >
      {isEmpty ? DASH : formatAmount(amount)}
    </td>
  )
}
