import type { BadgeColorMap } from '~/lib/badge-colors'
import { Badge } from '~/components/ui/badge'
import { getBadgeColor } from '~/lib/badge-colors'
import { cn } from '~/lib/utils'

interface StatusBadgeProps {
  value: string
  label?: string
  colorMap?: BadgeColorMap
  className?: string
}

export function StatusBadge({ value, label, colorMap, className }: StatusBadgeProps) {
  return (
    <Badge
      variant="outline"
      className={cn('capitalize', getBadgeColor(value, colorMap), className)}
    >
      {label ?? value.toLowerCase().replace(/_/g, ' ')}
    </Badge>
  )
}
