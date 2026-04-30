import { formatAmount } from '~/lib/formatters'

import { cn } from '~/lib/utils'
import { EDGE_SHADOW, Z } from './sticky'

export type MatrixTotalVariant
  = | 'row-total'
    | 'col-total'
    | 'grand-total'
    | 'row-subtotal'
    | 'col-subtotal'
    | 'subtotal-intersect'

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
