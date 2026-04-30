import type { Uuid } from '../types'
import { formatAmount } from '~/lib/formatters'
import { cn } from '~/lib/utils'

export interface MatrixCellProps {
  productId: Uuid
  storageId: Uuid
  amount: number | undefined
  ariaLabel?: string
  onClick?: () => void
}

const DASH = '\u2014'

export function MatrixCell({ amount, ariaLabel, onClick }: MatrixCellProps) {
  const isEmpty = amount == null || amount === 0
  return (
    <td className={cn('text-right tabular-nums px-[var(--cell-px)] py-[var(--cell-py)] border-b', isEmpty && 'text-muted-foreground')}>
      {isEmpty
        ? DASH
        : (
            <button
              type="button"
              className="hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring rounded-sm"
              aria-label={ariaLabel}
              onClick={onClick}
            >
              {formatAmount(amount)}
            </button>
          )}
    </td>
  )
}
