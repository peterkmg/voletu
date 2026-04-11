import type { TableDensity } from './density-state'
import { Rows2, Rows3, Rows4 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { cn } from '~/lib/utils'
import { useTableDensity } from './density-state'

export type { TableDensity } from './density-state'

export function useDensity() {
  return useTableDensity()
}

export const densityClasses: Record<TableDensity, string> = {
  compact: 'py-1',
  normal: 'py-2',
  comfortable: 'py-3',
}

// ── Density Toggle ───────────────────────────────

type DensityLabelKey = 'densityCompact' | 'densityNormal' | 'densityComfortable'

export const tableDensityOptions: { value: TableDensity, icon: typeof Rows3, labelKey: DensityLabelKey }[] = [
  { value: 'compact', icon: Rows4, labelKey: 'densityCompact' },
  { value: 'normal', icon: Rows3, labelKey: 'densityNormal' },
  { value: 'comfortable', icon: Rows2, labelKey: 'densityComfortable' },
]

export function DensityToggle() {
  const { density, setDensity } = useDensity()
  const { t } = useTranslation('common')

  return (
    <div className="flex items-center rounded-md border">
      {tableDensityOptions.map(({ value, icon: Icon, labelKey }) => (
        <Tooltip key={value}>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className={cn(
                'h-8 w-8 rounded-none first:rounded-l-md last:rounded-r-md',
                density === value && 'bg-accent',
              )}
              onClick={() => setDensity(value)}
            >
              <Icon className="h-4 w-4" />
              <span className="sr-only">{t(`table.${labelKey}`)}</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>{t(`table.${labelKey}`)}</TooltipContent>
        </Tooltip>
      ))}
    </div>
  )
}
