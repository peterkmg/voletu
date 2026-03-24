import { List, ScrollText } from 'lucide-react'
import { Button } from '~/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { cn } from '~/lib/utils'

export type TableMode = 'virtual' | 'paginated'

const modeOptions: { value: TableMode, icon: typeof List }[] = [
  { value: 'virtual', icon: ScrollText },
  { value: 'paginated', icon: List },
]

interface TableModeToggleProps {
  mode: TableMode
  onModeChange: (mode: TableMode) => void
}

export function TableModeToggle({ mode, onModeChange }: TableModeToggleProps) {
  return (
    <div className="flex items-center rounded-md border">
      {modeOptions.map(({ value, icon: Icon }) => (
        <Tooltip key={value}>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className={cn(
                'h-8 w-8 rounded-none first:rounded-l-md last:rounded-r-md',
                mode === value && 'bg-accent',
              )}
              onClick={() => onModeChange(value)}
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
