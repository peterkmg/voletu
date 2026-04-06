import type { Column } from '@tanstack/react-table'
import { ChevronRight, Filter } from 'lucide-react'
import { useMemo, useState } from 'react'
import { Button } from '~/components/ui/button'
import { Checkbox } from '~/components/ui/checkbox'
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '~/components/ui/collapsible'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '~/components/ui/command'
import { DebouncedInput } from '~/components/ui/debounced-input'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '~/components/ui/popover'
import { cn } from '~/lib/utils'

type FilterType = 'text' | 'date' | 'number' | 'enum'

export function detectFilterType(facetedValues: Map<unknown, number>): FilterType {
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

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/** Compute "Select All" checkbox state for a set of values. */
function selectAllState(allValues: string[], selectedSet: Set<string>): boolean | 'indeterminate' {
  if (allValues.length === 0) return false
  const count = allValues.filter(v => selectedSet.has(v)).length
  if (count === 0) return false
  if (count === allValues.length) return true
  return 'indeterminate'
}

/** Get the effective selected set: when no filter is active, treat ALL values as selected. */
function getSelectedSet(filterValue: unknown, allValues: string[]): Set<string> {
  if (filterValue === undefined || filterValue === null)
    return new Set(allValues)
  return new Set(filterValue as string[])
}

/** Update filter value: if all values selected → clear to undefined; otherwise set array. */
function commitFilter<TData, TValue>(column: Column<TData, TValue>, next: Set<string>, allValues: string[]) {
  if (allValues.length > 0 && allValues.every(v => next.has(v)))
    column.setFilterValue(undefined)
  else
    column.setFilterValue(Array.from(next))
}

// ---------------------------------------------------------------------------
// Text/Enum Filter — checkbox list with Select All, all-selected-by-default
// ---------------------------------------------------------------------------

export function TextEnumFilterContent<TData, TValue>({
  column,
  searchPlaceholder = 'Search...',
  emptyMessage = 'No values found.',
}: {
  column: Column<TData, TValue>
  searchPlaceholder?: string
  emptyMessage?: string
}) {
  const facets = column.getFacetedUniqueValues()
  const filterValue = column.getFilterValue() as string[] | undefined

  const sortedOptions = useMemo(() => {
    const entries: { label: string, value: string, count: number }[] = []
    for (const [value, count] of facets) {
      if (value == null || value === '')
        continue
      entries.push({ label: String(value), value: String(value), count })
    }
    return entries.sort((a, b) => a.label.localeCompare(b.label))
  }, [facets])

  const allValues = useMemo(() => sortedOptions.map(o => o.value), [sortedOptions])
  const selectedSet = getSelectedSet(filterValue, allValues)
  const isFiltered = filterValue !== undefined

  function toggle(value: string) {
    const next = new Set(selectedSet)
    if (next.has(value)) next.delete(value)
    else next.add(value)
    commitFilter(column, next, allValues)
  }

  function toggleSelectAll() {
    const allSelected = allValues.every(v => selectedSet.has(v))
    if (allSelected)
      column.setFilterValue([])
    else
      column.setFilterValue(undefined)
  }

  return (
    <Command>
      <CommandInput placeholder={searchPlaceholder} className="h-8 text-xs" />
      <CommandList>
        <CommandEmpty className="text-xs py-4">{emptyMessage}</CommandEmpty>
        <CommandGroup>
          {/* Select All */}
          <CommandItem onSelect={toggleSelectAll} className="text-xs font-medium">
            <Checkbox
              checked={selectAllState(allValues, selectedSet)}
              className="size-3.5 pointer-events-none"
              tabIndex={-1}
            />
            <span>Select All</span>
          </CommandItem>
          <CommandSeparator className="my-1" />
          {sortedOptions.map(option => (
            <CommandItem key={option.value} onSelect={() => toggle(option.value)} className="text-xs">
              <Checkbox
                checked={selectedSet.has(option.value)}
                className="size-3.5 pointer-events-none"
                tabIndex={-1}
              />
              <span className="truncate">{option.label}</span>
              <span className="ms-auto font-mono text-[10px] text-muted-foreground">
                {option.count}
              </span>
            </CommandItem>
          ))}
        </CommandGroup>
        {isFiltered && (
          <>
            <CommandSeparator />
            <CommandGroup>
              <CommandItem
                onSelect={() => column.setFilterValue(undefined)}
                className="justify-center text-center text-xs"
              >
                Clear filter
              </CommandItem>
            </CommandGroup>
          </>
        )}
      </CommandList>
    </Command>
  )
}

// ---------------------------------------------------------------------------
// Date Filter — 3-level Year → Month → Day tree, all-selected-by-default
// ---------------------------------------------------------------------------

interface DayNode { date: string, count: number }
interface MonthNode { year: number, month: number, label: string, days: DayNode[], totalCount: number }
interface YearNode { year: number, months: MonthNode[], totalCount: number }

function buildDateTree(facets: Map<unknown, number>): { years: YearNode[], allDates: string[] } {
  // Normalize ISO dates to YYYY-MM-DD and aggregate
  const dayMap = new Map<string, number>()
  for (const [value, count] of facets) {
    if (value == null) continue
    const dateStr = String(value).slice(0, 10) // "2026-03-31"
    dayMap.set(dateStr, (dayMap.get(dateStr) ?? 0) + count)
  }

  // Build hierarchy
  const yearMap = new Map<number, Map<number, DayNode[]>>()
  for (const [dateStr, count] of dayMap) {
    const parts = dateStr.split('-')
    const y = Number(parts[0])
    const m = Number(parts[1])
    if (!yearMap.has(y)) yearMap.set(y, new Map())
    const monthMap = yearMap.get(y)!
    if (!monthMap.has(m)) monthMap.set(m, [])
    monthMap.get(m)!.push({ date: dateStr, count })
  }

  const allDates: string[] = []
  const years: YearNode[] = []

  for (const [year, monthMap] of yearMap) {
    const months: MonthNode[] = []
    let yearTotal = 0
    for (const [month, days] of monthMap) {
      days.sort((a, b) => b.date.localeCompare(a.date))
      const totalCount = days.reduce((s, d) => s + d.count, 0)
      yearTotal += totalCount
      const label = new Date(year, month - 1).toLocaleString('default', { month: 'short' })
      months.push({ year, month, label, days, totalCount })
      for (const d of days) allDates.push(d.date)
    }
    months.sort((a, b) => b.month - a.month)
    years.push({ year, months, totalCount: yearTotal })
  }
  years.sort((a, b) => b.year - a.year)
  return { years, allDates }
}

export function DateGroupFilterContent<TData, TValue>({
  column,
}: {
  column: Column<TData, TValue>
}) {
  const facets = column.getFacetedUniqueValues()
  const filterValue = column.getFilterValue() as string[] | undefined
  const [search, setSearch] = useState('')

  const { years, allDates } = useMemo(() => buildDateTree(facets), [facets])
  const selectedSet = getSelectedSet(filterValue, allDates)
  const isFiltered = filterValue !== undefined
  const currentYear = new Date().getFullYear()
  const hasSearch = search.trim().length > 0

  // --- Toggle functions ---

  function toggleDay(date: string) {
    const next = new Set(selectedSet)
    if (next.has(date)) next.delete(date)
    else next.add(date)
    commitFilter(column, next, allDates)
  }

  function toggleMonth(monthNode: MonthNode) {
    const dates = monthNode.days.map(d => d.date)
    const allSelected = dates.every(d => selectedSet.has(d))
    const next = new Set(selectedSet)
    if (allSelected) {
      for (const d of dates) next.delete(d)
    }
    else {
      for (const d of dates) next.add(d)
    }
    commitFilter(column, next, allDates)
  }

  function toggleYear(yearNode: YearNode) {
    const dates = yearNode.months.flatMap(m => m.days.map(d => d.date))
    const allSelected = dates.every(d => selectedSet.has(d))
    const next = new Set(selectedSet)
    if (allSelected) {
      for (const d of dates) next.delete(d)
    }
    else {
      for (const d of dates) next.add(d)
    }
    commitFilter(column, next, allDates)
  }

  function toggleSelectAll() {
    const allSelected = allDates.every(d => selectedSet.has(d))
    if (allSelected)
      column.setFilterValue([])
    else
      column.setFilterValue(undefined)
  }

  // --- Tri-state helpers ---

  function monthChecked(m: MonthNode): boolean | 'indeterminate' {
    return selectAllState(m.days.map(d => d.date), selectedSet)
  }

  function yearChecked(y: YearNode): boolean | 'indeterminate' {
    return selectAllState(y.months.flatMap(m => m.days.map(d => d.date)), selectedSet)
  }

  return (
    <Command shouldFilter={false}>
      <CommandInput placeholder="Search..." value={search} onValueChange={setSearch} className="h-8 text-xs" />
      <CommandList>
        <CommandEmpty className="text-xs py-4">No dates found.</CommandEmpty>
        <CommandGroup>
          {/* Select All */}
          <CommandItem onSelect={toggleSelectAll} className="text-xs font-medium">
            <Checkbox
              checked={selectAllState(allDates, selectedSet)}
              className="size-3.5 pointer-events-none"
              tabIndex={-1}
            />
            <span>Select All</span>
          </CommandItem>
        </CommandGroup>
        <CommandSeparator className="my-0.5" />

        {years.map((yearNode) => {
          const yearMatches = !hasSearch || yearNode.months.some(m =>
            m.label.toLowerCase().includes(search.toLowerCase())
            || String(yearNode.year).includes(search)
            || m.days.some(d => d.date.includes(search)),
          )
          if (!yearMatches) return null

          return (
            <Collapsible
              key={yearNode.year}
              defaultOpen={yearNode.year === currentYear}
              open={hasSearch ? true : undefined}
            >
              <div className="flex items-center gap-1 px-2 py-1">
                <CollapsibleTrigger className="flex size-4 items-center justify-center rounded-sm hover:bg-accent">
                  <ChevronRight className="size-3 transition-transform duration-200 [[data-state=open]>&]:rotate-90" />
                </CollapsibleTrigger>
                <Checkbox
                  checked={yearChecked(yearNode)}
                  onCheckedChange={() => toggleYear(yearNode)}
                  className="size-3.5"
                />
                <span className="text-xs font-medium">{yearNode.year}</span>
                <span className="ms-auto font-mono text-[10px] text-muted-foreground">
                  {yearNode.totalCount}
                </span>
              </div>
              <CollapsibleContent>
                {yearNode.months.map((monthNode) => {
                  const monthMatches = !hasSearch
                    || monthNode.label.toLowerCase().includes(search.toLowerCase())
                    || monthNode.days.some(d => d.date.includes(search))
                  if (!monthMatches) return null

                  return (
                    <Collapsible key={monthNode.month} open={hasSearch ? true : undefined}>
                      <div className="flex items-center gap-1 ps-6 pe-2 py-0.5">
                        <CollapsibleTrigger className="flex size-4 items-center justify-center rounded-sm hover:bg-accent">
                          <ChevronRight className="size-3 transition-transform duration-200 [[data-state=open]>&]:rotate-90" />
                        </CollapsibleTrigger>
                        <Checkbox
                          checked={monthChecked(monthNode)}
                          onCheckedChange={() => toggleMonth(monthNode)}
                          className="size-3.5"
                        />
                        <span className="text-xs">{monthNode.label}</span>
                        <span className="ms-auto font-mono text-[10px] text-muted-foreground">
                          {monthNode.totalCount}
                        </span>
                      </div>
                      <CollapsibleContent>
                        <CommandGroup>
                          {monthNode.days.map(day => (
                            <CommandItem
                              key={day.date}
                              value={day.date}
                              onSelect={() => toggleDay(day.date)}
                              className="ps-12 py-0.5 text-xs"
                            >
                              <Checkbox
                                checked={selectedSet.has(day.date)}
                                className="size-3.5 pointer-events-none"
                                tabIndex={-1}
                              />
                              <span>{day.date.slice(8)}</span>
                              <span className="ms-auto font-mono text-[10px] text-muted-foreground">
                                {day.count}
                              </span>
                            </CommandItem>
                          ))}
                        </CommandGroup>
                      </CollapsibleContent>
                    </Collapsible>
                  )
                })}
              </CollapsibleContent>
            </Collapsible>
          )
        })}

        {isFiltered && (
          <>
            <CommandSeparator />
            <CommandGroup>
              <CommandItem
                onSelect={() => column.setFilterValue(undefined)}
                className="justify-center text-center text-xs"
              >
                Clear filter
              </CommandItem>
            </CommandGroup>
          </>
        )}
      </CommandList>
    </Command>
  )
}

// ---------------------------------------------------------------------------
// Number Range Filter — min/max inputs
// ---------------------------------------------------------------------------

export function NumberRangeFilterContent<TData, TValue>({
  column,
}: {
  column: Column<TData, TValue>
}) {
  const filterValue = (column.getFilterValue() as [number?, number?]) ?? [undefined, undefined]
  const hasFilter = filterValue[0] != null || filterValue[1] != null

  return (
    <div className="space-y-2 p-2">
      <div className="space-y-1">
        <label className="text-[10px] font-medium text-muted-foreground">Min</label>
        <DebouncedInput
          type="number"
          value={filterValue[0]?.toString() ?? ''}
          onChange={(val) => {
            const min = val === '' ? undefined : Number(val)
            column.setFilterValue([min, filterValue[1]])
          }}
          className="h-7 text-xs"
          placeholder="Min value"
        />
      </div>
      <div className="space-y-1">
        <label className="text-[10px] font-medium text-muted-foreground">Max</label>
        <DebouncedInput
          type="number"
          value={filterValue[1]?.toString() ?? ''}
          onChange={(val) => {
            const max = val === '' ? undefined : Number(val)
            column.setFilterValue([filterValue[0], max])
          }}
          className="h-7 text-xs"
          placeholder="Max value"
        />
      </div>
      {hasFilter && (
        <Button
          variant="ghost"
          size="sm"
          className="h-6 w-full text-xs"
          onClick={() => column.setFilterValue(undefined)}
        >
          Clear
        </Button>
      )}
    </div>
  )
}

// ---------------------------------------------------------------------------
// ColumnFilterInline — auto-detecting inline filter for the merged header
// ---------------------------------------------------------------------------

export function ColumnFilterInline<TData, TValue>({
  column,
}: {
  column: Column<TData, TValue>
}) {
  const meta = column.columnDef.meta
  if (meta?.enableHeaderFilter === false || !column.getCanFilter())
    return null

  const facets = column.getFacetedUniqueValues()
  const filterType = meta?.filterType ?? detectFilterType(facets)

  if (filterType === 'number')
    return <NumberRangeFilterContent column={column} />
  if (filterType === 'date')
    return <DateGroupFilterContent column={column} />
  return <TextEnumFilterContent column={column} />
}

// ---------------------------------------------------------------------------
// Legacy ColumnFilter (standalone Popover wrapper)
// ---------------------------------------------------------------------------

/** @deprecated Use ColumnFilterInline inside the merged column header dropdown instead. */
export function ColumnFilter<TData, TValue>({
  column,
}: {
  column: Column<TData, TValue>
}) {
  const meta = column.columnDef.meta
  const enableFilter = meta?.enableHeaderFilter !== false

  if (!enableFilter || !column.getCanFilter())
    return null

  const facets = column.getFacetedUniqueValues()
  const filterType = meta?.filterType ?? detectFilterType(facets)
  const hasFilter = (() => {
    const val = column.getFilterValue()
    if (val == null) return false
    if (Array.isArray(val)) return val.length > 0
    return true
  })()

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="sm"
          className={cn('h-6 w-6 p-0', hasFilter && 'text-primary')}
        >
          <Filter className="size-3" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[220px] p-0" align="start">
        {filterType === 'number'
          ? <NumberRangeFilterContent column={column} />
          : filterType === 'date'
            ? <DateGroupFilterContent column={column} />
            : <TextEnumFilterContent column={column} />}
      </PopoverContent>
    </Popover>
  )
}
