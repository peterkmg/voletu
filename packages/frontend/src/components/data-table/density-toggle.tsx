import type { TableDensity } from './density-context'
import { AlignJustify, List, StretchHorizontal } from 'lucide-react'
import { Button } from '~/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { cn } from '~/lib/utils'
import { useDensity } from './density-context'

const densityOptions: { value: TableDensity, icon: typeof List }[] = [
  { value: 'compact', icon: AlignJustify },
  { value: 'normal', icon: List },
  { value: 'comfortable', icon: StretchHorizontal },
]

export function DensityToggle() {
  const { density, setDensity } = useDensity()

  return (
    <div className="flex items-center rounded-md border">
      {densityOptions.map(({ value, icon: Icon }) => (
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
              <span className="sr-only">{value}</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>{value}</TooltipContent>
        </Tooltip>
      ))}
    </div>
  )
}
