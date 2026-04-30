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

type DensityLabelKey = 'tables:density.compact' | 'tables:density.normal' | 'tables:density.comfortable'

export const tableDensityOptions: { value: TableDensity, icon: typeof Rows3, labelKey: DensityLabelKey }[] = [
  { value: 'compact', icon: Rows4, labelKey: 'tables:density.compact' },
  { value: 'normal', icon: Rows3, labelKey: 'tables:density.normal' },
  { value: 'comfortable', icon: Rows2, labelKey: 'tables:density.comfortable' },
]

export function DensityToggle() {
  const { density, setDensity } = useDensity()
  const { t } = useTranslation('tables')

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
              <span className="sr-only">{t(labelKey)}</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>{t(labelKey)}</TooltipContent>
        </Tooltip>
      ))}
    </div>
  )
}
