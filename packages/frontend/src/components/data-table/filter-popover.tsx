import { Check, Filter } from 'lucide-react'
import { useState } from 'react'
import { Button } from '~/components/ui/button'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '~/components/ui/command'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '~/components/ui/popover'
import { cn } from '~/lib/utils'

export interface FilterOption {
  label: string
  value: string
  count: number
}

interface FilterPopoverProps {
  hasFilter: boolean
  searchPlaceholder?: string
  emptyMessage?: string
  width?: string
  options: FilterOption[]
  selectedValues: Set<string>
  onSelect: (value: string) => void
  onClear: () => void
}

export function FilterPopover({
  hasFilter,
  searchPlaceholder = 'Search...',
  emptyMessage = 'No values found.',
  width = 'w-[220px]',
  options,
  selectedValues,
  onSelect,
  onClear,
}: FilterPopoverProps) {
  const [open, setOpen] = useState(false)

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="sm"
          className={cn(
            'h-6 w-6 p-0',
            hasFilter && 'text-primary',
          )}
        >
          <Filter className="size-3" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className={cn(width, 'p-0')} align="start">
        <Command>
          <CommandInput placeholder={searchPlaceholder} />
          <CommandList>
            <CommandEmpty>{emptyMessage}</CommandEmpty>
            <CommandGroup>
              {options.map(option => (
                <CommandItem
                  key={option.value}
                  onSelect={() => onSelect(option.value)}
                >
                  <div
                    className={cn(
                      'flex size-4 items-center justify-center rounded-sm border border-primary',
                      selectedValues.has(option.value)
                        ? 'bg-primary text-primary-foreground'
                        : 'opacity-50 [&_svg]:invisible',
                    )}
                  >
                    <Check className="size-3 text-background" />
                  </div>
                  <span className="truncate">{option.label}</span>
                  <span className="ms-auto font-mono text-xs text-muted-foreground">
                    {option.count}
                  </span>
                </CommandItem>
              ))}
            </CommandGroup>
            {hasFilter && (
              <>
                <CommandSeparator />
                <CommandGroup>
                  <CommandItem
                    onSelect={onClear}
                    className="justify-center text-center"
                  >
                    Clear filter
                  </CommandItem>
                </CommandGroup>
              </>
            )}
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  )
}
