import { Skeleton } from '~/components/ui/skeleton'
import { TableCell, TableRow } from '~/components/ui/table'
import { SKELETON_ROWS } from '~/lib/ux-constants'

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
