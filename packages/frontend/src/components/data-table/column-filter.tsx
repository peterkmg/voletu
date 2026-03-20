import type { Column } from '@tanstack/react-table'
import { Check, Filter } from 'lucide-react'
import { useMemo, useState } from 'react'
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
import { DebouncedInput } from './debounced-input'

type FilterType = 'text' | 'date' | 'number' | 'enum'

interface ColumnFilterProps<TData, TValue> {
  column: Column<TData, TValue>
}

function detectFilterType(facetedValues: Map<unknown, number>): FilterType {
  for (const [value] of facetedValues) {
    if (value == null)
      continue
    if (typeof value === 'number')
      return 'number'
    if (typeof value === 'string') {
      if (/^\d{4}-\d{2}-\d{2}/.test(value))
        return 'date'
    }
    break
  }
  return 'text'
}

function formatMonthYear(dateStr: string): string {
  const d = new Date(dateStr)
  return `${d.toLocaleString('default', { month: 'short' })} ${d.getFullYear()}`
}

export function ColumnFilter<TData, TValue>({
  column,
}: ColumnFilterProps<TData, TValue>) {
  const meta = column.columnDef.meta
  const enableFilter = meta?.enableHeaderFilter !== false

  if (!enableFilter || !column.getCanFilter())
    return null

  const facets = column.getFacetedUniqueValues()
  const filterType = meta?.filterType ?? detectFilterType(facets)

  if (filterType === 'number') {
    return <NumberRangeFilter column={column} />
  }

  if (filterType === 'date') {
    return <DateGroupFilter column={column} facets={facets} />
  }

  return <TextEnumFilter column={column} facets={facets} />
}

/** Text/enum filter — shows unique values as checkboxes */
function TextEnumFilter<TData, TValue>({
  column,
  facets,
}: {
  column: Column<TData, TValue>
  facets: Map<unknown, number>
}) {
  const [open, setOpen] = useState(false)
  const selectedValues = new Set(column.getFilterValue() as string[] | undefined)
  const hasFilter = selectedValues.size > 0

  const sortedOptions = useMemo(() => {
    const entries: { label: string, value: string, count: number }[] = []
    for (const [value, count] of facets) {
      if (value == null || value === '')
        continue
      entries.push({ label: String(value), value: String(value), count })
    }
    return entries.sort((a, b) => b.count - a.count)
  }, [facets])

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
      <PopoverContent className="w-[220px] p-0" align="start">
        <Command>
          <CommandInput placeholder="Search..." />
          <CommandList>
            <CommandEmpty>No values found.</CommandEmpty>
            <CommandGroup>
              {sortedOptions.map(option => (
                <CommandItem
                  key={option.value}
                  onSelect={() => {
                    if (selectedValues.has(option.value)) {
                      selectedValues.delete(option.value)
                    }
                    else {
                      selectedValues.add(option.value)
                    }
                    const values = Array.from(selectedValues)
                    column.setFilterValue(values.length ? values : undefined)
                  }}
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
                    onSelect={() => column.setFilterValue(undefined)}
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

/** Date filter — groups by month/year */
function DateGroupFilter<TData, TValue>({
  column,
  facets,
}: {
  column: Column<TData, TValue>
  facets: Map<unknown, number>
}) {
  const [open, setOpen] = useState(false)
  const selectedValues = new Set(column.getFilterValue() as string[] | undefined)
  const hasFilter = selectedValues.size > 0

  const monthOptions = useMemo(() => {
    const groups = new Map<string, number>()
    for (const [value, count] of facets) {
      if (value == null)
        continue
      const key = formatMonthYear(String(value))
      groups.set(key, (groups.get(key) ?? 0) + count)
    }
    return Array.from(groups, ([label, count]) => ({ label, value: label, count }))
      .sort((a, b) => {
        const da = new Date(a.label)
        const db = new Date(b.label)
        return db.getTime() - da.getTime()
      })
  }, [facets])

  // Custom filter function that matches by month/year
  // We store selected month labels and filter in the column's filterFn
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
      <PopoverContent className="w-[200px] p-0" align="start">
        <Command>
          <CommandInput placeholder="Search month..." />
          <CommandList>
            <CommandEmpty>No dates found.</CommandEmpty>
            <CommandGroup>
              {monthOptions.map(option => (
                <CommandItem
                  key={option.value}
                  onSelect={() => {
                    if (selectedValues.has(option.value)) {
                      selectedValues.delete(option.value)
                    }
                    else {
                      selectedValues.add(option.value)
                    }
                    const values = Array.from(selectedValues)
                    column.setFilterValue(values.length ? values : undefined)
                  }}
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
                  <span>{option.label}</span>
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
                    onSelect={() => column.setFilterValue(undefined)}
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

/** Number range filter — min/max inputs */
function NumberRangeFilter<TData, TValue>({
  column,
}: {
  column: Column<TData, TValue>
}) {
  const [open, setOpen] = useState(false)
  const filterValue = (column.getFilterValue() as [number?, number?]) ?? [undefined, undefined]
  const hasFilter = filterValue[0] != null || filterValue[1] != null

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
      <PopoverContent className="w-[200px] space-y-2 p-3" align="start">
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">Min</label>
          <DebouncedInput
            type="number"
            value={filterValue[0]?.toString() ?? ''}
            onChange={(val) => {
              const min = val === '' ? undefined : Number(val)
              column.setFilterValue([min, filterValue[1]])
            }}
            className="h-8"
            placeholder="Min value"
          />
        </div>
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">Max</label>
          <DebouncedInput
            type="number"
            value={filterValue[1]?.toString() ?? ''}
            onChange={(val) => {
              const max = val === '' ? undefined : Number(val)
              column.setFilterValue([filterValue[0], max])
            }}
            className="h-8"
            placeholder="Max value"
          />
        </div>
        {hasFilter && (
          <Button
            variant="ghost"
            size="sm"
            className="h-7 w-full"
            onClick={() => column.setFilterValue(undefined)}
          >
            Clear
          </Button>
        )}
      </PopoverContent>
    </Popover>
  )
}
