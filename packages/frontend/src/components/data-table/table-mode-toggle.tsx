import { LayoutList, ScrollText } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import { cn } from '~/lib/utils'

export type TableMode = 'virtual' | 'paginated'

const modeOptions: { value: TableMode, icon: typeof ScrollText, label: string }[] = [
  { value: 'virtual', icon: ScrollText, label: 'table.modeVirtual' },
  { value: 'paginated', icon: LayoutList, label: 'table.modePaginated' },
]

interface TableModeToggleProps {
  mode: TableMode
  onModeChange: (mode: TableMode) => void
}

export function TableModeToggle({ mode, onModeChange }: TableModeToggleProps) {
  const { t } = useTranslation('common')

  return (
    <div className="flex items-center rounded-md border">
      {modeOptions.map(({ value, icon: Icon, label }) => (
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
              <span className="sr-only">{t(label)}</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>{t(label)}</TooltipContent>
        </Tooltip>
      ))}
    </div>
  )
}
