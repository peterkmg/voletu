import { Inbox } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Skeleton } from '~/components/ui/skeleton'
import { TableCell, TableRow } from '~/components/ui/table'
import { SKELETON_ROWS } from '~/lib/ux-constants'

// ── Empty State ──────────────────────────────────

interface EmptyStateProps {
  colSpan: number
  message?: string
  icon?: React.ReactNode
  action?: React.ReactNode
}

export function EmptyState({ colSpan, message, icon, action }: EmptyStateProps) {
  const { t } = useTranslation('common')
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className="h-32">
        <div className="flex flex-col items-center justify-center gap-2 text-muted-foreground">
          {icon ?? <Inbox className="h-8 w-8" />}
          <span className="text-sm">{message ?? t('table.noResults')}</span>
          {action}
        </div>
      </TableCell>
    </TableRow>
  )
}

// ── Table Skeleton ───────────────────────────────

interface TableSkeletonProps {
  columns: number
  rows?: number
  densityCls?: string
}

export function TableSkeleton({ columns, rows = SKELETON_ROWS, densityCls }: TableSkeletonProps) {
  return (
    <>
      {Array.from({ length: rows }, (_, i) => (
        <TableRow key={i}>
          {Array.from({ length: columns }, (_, j) => (
            <TableCell key={j} className={densityCls}>
              <Skeleton className="h-4 w-full" />
            </TableCell>
          ))}
        </TableRow>
      ))}
    </>
  )
}
