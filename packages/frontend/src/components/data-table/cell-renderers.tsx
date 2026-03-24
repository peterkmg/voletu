import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { formatDate, formatDateTime, truncateId } from '~/lib/formatters'

export function NullCell() {
  return <span className="text-muted-foreground">&mdash;</span>
}

export function DateCell({ value }: { value: string | null | undefined }) {
  if (!value)
    return <NullCell />
  return (
    <span className="text-sm tabular-nums text-muted-foreground">
      {formatDate(value)}
    </span>
  )
}

export function DateTimeCell({ value }: { value: string | null | undefined }) {
  if (!value)
    return <NullCell />
  return (
    <span className="text-sm tabular-nums text-muted-foreground">
      {formatDateTime(value)}
    </span>
  )
}

export function NumericCell({
  value,
  padWidth,
}: {
  value: number | string | null | undefined
  padWidth?: number
}) {
  if (value == null)
    return <NullCell />
  const display = padWidth
    ? String(value).padStart(padWidth, '0')
    : String(value)
  return (
    <span className="text-sm tabular-nums">{display}</span>
  )
}

export function IdCell({ value }: { value: string | null | undefined }) {
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
}

export function ResolvedCell({ value }: { value: string | null | undefined }) {
  if (!value)
    return <NullCell />
  return <span>{value}</span>
}

export function LookupCell({
  value,
  lookupMap,
}: {
  value: string | null | undefined
  lookupMap: Map<string, string>
}) {
  if (!value)
    return <NullCell />
  const resolved = lookupMap.get(value)
  if (resolved) {
    return <span>{resolved}</span>
  }
  return <IdCell value={value} />
}
