import type { TableDensity } from './density-context'
import { Rows2, Rows3, Rows4 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { cn } from '~/lib/utils'
import { useDensity } from './density-context'

const densityOptions: { value: TableDensity, icon: typeof Rows3, label: string }[] = [
  { value: 'compact', icon: Rows4, label: 'table.densityCompact' },
  { value: 'normal', icon: Rows3, label: 'table.densityNormal' },
  { value: 'comfortable', icon: Rows2, label: 'table.densityComfortable' },
]

export function DensityToggle() {
  const { density, setDensity } = useDensity()
  const { t } = useTranslation('common')

  return (
    <div className="flex items-center rounded-md border">
      {densityOptions.map(({ value, icon: Icon, label }) => (
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
              <span className="sr-only">{t(label)}</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>{t(label)}</TooltipContent>
        </Tooltip>
      ))}
    </div>
  )
}
