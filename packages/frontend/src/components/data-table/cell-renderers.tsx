import { memo } from 'react'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { formatAmount, formatDate, formatDateTime, truncateId } from '~/lib/formatters'
import { cn } from '~/lib/utils'

export const NullCell = memo(() => (
  <span className="text-muted-foreground">&mdash;</span>
))

export const DateCell = memo(({ value }: { value: string | null | undefined }) => {
  if (!value)
    return <NullCell />
  return (
    <span className="text-sm tabular-nums text-muted-foreground">
      {formatDate(value)}
    </span>
  )
})

export const DateTimeCell = memo(({ value }: { value: string | null | undefined }) => {
  if (!value)
    return <NullCell />
  return (
    <span className="text-sm tabular-nums text-muted-foreground">
      {formatDateTime(value)}
    </span>
  )
})

export const NumericCell = memo(({
  value,
  padWidth,
  unit,
}: {
  value: number | string | null | undefined
  padWidth?: number
  unit?: string
}) => {
  if (value == null)
    return <NullCell />
  const num = typeof value === 'string' ? Number.parseFloat(value) : value
  const display = padWidth
    ? String(value).padStart(padWidth, '0')
    : formatAmount(value, unit)
  const colorClass = Number.isNaN(num) || num === 0
    ? undefined
    : num < 0
      ? 'text-red-600 dark:text-red-400'
      : 'text-green-700 dark:text-green-400'
  return (
    <span className={cn('text-sm font-medium tabular-nums', colorClass)}>{display}</span>
  )
})

export const IdCell = memo(({ value }: { value: string | null | undefined }) => {
  if (!value)
    return <NullCell />
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span className="cursor-default font-mono text-xs text-muted-foreground">
          {truncateId(value)}
        </span>
      </TooltipTrigger>
      <TooltipContent>
        <span className="font-mono text-xs">{value}</span>
      </TooltipContent>
    </Tooltip>
  )
})

export const ResolvedCell = memo(({ value }: { value: string | null | undefined }) => {
  if (!value)
    return <NullCell />
  return <span>{value}</span>
})

export const LookupCell = memo(({
  value,
  lookupMap,
}: {
  value: string | null | undefined
  lookupMap: Map<string, string>
}) => {
  if (!value)
    return <NullCell />
  const resolved = lookupMap.get(value)
  if (resolved) {
    return <span>{resolved}</span>
  }
  return <IdCell value={value} />
})
