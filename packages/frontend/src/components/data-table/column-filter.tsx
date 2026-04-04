import type { Column } from '@tanstack/react-table'
import { Filter } from 'lucide-react'
import { useMemo, useState } from 'react'
import { Button } from '~/components/ui/button'
import { DebouncedInput } from '~/components/ui/debounced-input'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '~/components/ui/popover'
import { cn } from '~/lib/utils'
import { FilterPopover } from './filter-popover'

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
  const selectedValues = new Set(column.getFilterValue() as string[] | undefined)

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
    <FilterPopover
      hasFilter={selectedValues.size > 0}
      options={sortedOptions}
      selectedValues={selectedValues}
      onSelect={(value) => {
        if (selectedValues.has(value)) {
          selectedValues.delete(value)
        }
        else {
          selectedValues.add(value)
        }
        const values = Array.from(selectedValues)
        column.setFilterValue(values.length ? values : undefined)
      }}
      onClear={() => column.setFilterValue(undefined)}
    />
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
  const selectedValues = new Set(column.getFilterValue() as string[] | undefined)

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
    <FilterPopover
      hasFilter={selectedValues.size > 0}
      searchPlaceholder="Search month..."
      emptyMessage="No dates found."
      width="w-[200px]"
      options={monthOptions}
      selectedValues={selectedValues}
      onSelect={(value) => {
        if (selectedValues.has(value)) {
          selectedValues.delete(value)
        }
        else {
          selectedValues.add(value)
        }
        const values = Array.from(selectedValues)
        column.setFilterValue(values.length ? values : undefined)
      }}
      onClear={() => column.setFilterValue(undefined)}
    />
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
