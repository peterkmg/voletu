import type { Column } from '@tanstack/react-table'
import type { MonthNode, YearNode } from './column-filter-state'
import { ChevronRight } from 'lucide-react'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
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
  buildDateTree,
  commitFilter,
  getSelectedSet,
  selectAllState,
} from './column-filter-state'
import { detectFilterType } from './filter-utils'

// ---------------------------------------------------------------------------
// Text/Enum Filter — checkbox list with Select All, all-selected-by-default
// ---------------------------------------------------------------------------

export function TextEnumFilterContent<TData, TValue>({
  column,
  searchPlaceholder,
  emptyMessage,
}: {
  column: Column<TData, TValue>
  searchPlaceholder?: string
  emptyMessage?: string
}) {
  const { t } = useTranslation('tables')
  const resolvedSearchPlaceholder = searchPlaceholder ?? t('tables:filter.search')
  const resolvedEmptyMessage = emptyMessage ?? t('tables:filter.noValues')
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
    if (next.has(value))
      next.delete(value)
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
      <CommandInput placeholder={resolvedSearchPlaceholder} className="h-8 text-xs" />
      <CommandList>
        <CommandEmpty className="text-xs py-4">{resolvedEmptyMessage}</CommandEmpty>
        <CommandGroup>
          {/* Select All */}
          <CommandItem onSelect={toggleSelectAll} className="text-xs font-medium">
            <Checkbox
              checked={selectAllState(allValues, selectedSet)}
              className="size-3.5 pointer-events-none"
              tabIndex={-1}
            />
            <span>{t('tables:selectAll')}</span>
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
                {t('tables:filter.clear')}
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

export function DateGroupFilterContent<TData, TValue>({
  column,
}: {
  column: Column<TData, TValue>
}) {
  const { t } = useTranslation('tables')
  const facets = column.getFacetedUniqueValues()
  const filterValue = column.getFilterValue() as string[] | undefined
  const [search, setSearch] = useState('')

  const { years, allDates } = useMemo(() => buildDateTree(facets), [facets])
  const selectedSet = getSelectedSet(filterValue, allDates)
  const isFiltered = filterValue !== undefined
  const currentYear = useMemo(() => new Date().getFullYear(), [])
  const hasSearch = search.trim().length > 0

  // --- Toggle functions ---

  function toggleDay(date: string) {
    const next = new Set(selectedSet)
    if (next.has(date))
      next.delete(date)
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
      <CommandInput placeholder={t('tables:filter.search')} value={search} onValueChange={setSearch} className="h-8 text-xs" />
      <CommandList>
        <CommandEmpty className="text-xs py-4">{t('tables:filter.noDates')}</CommandEmpty>
        <CommandGroup>
          {/* Select All */}
          <CommandItem onSelect={toggleSelectAll} className="text-xs font-medium">
            <Checkbox
              checked={selectAllState(allDates, selectedSet)}
              className="size-3.5 pointer-events-none"
              tabIndex={-1}
            />
            <span>{t('tables:selectAll')}</span>
          </CommandItem>
        </CommandGroup>
        <CommandSeparator className="my-0.5" />

        {years.map((yearNode) => {
          const yearMatches = !hasSearch || yearNode.months.some(m =>
            m.label.toLowerCase().includes(search.toLowerCase())
            || String(yearNode.year).includes(search)
            || m.days.some(d => d.date.includes(search)),
          )
          if (!yearMatches)
            return null

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
                  if (!monthMatches)
                    return null

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
                {t('tables:filter.clear')}
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
  const { t } = useTranslation('tables')
  const filterValue = (column.getFilterValue() as [number?, number?]) ?? [undefined, undefined]
  const hasFilter = filterValue[0] != null || filterValue[1] != null

  return (
    <div className="space-y-2 p-2">
      <div className="space-y-1">
        <label className="text-[10px] font-medium text-muted-foreground">{t('tables:filter.min')}</label>
        <DebouncedInput
          type="number"
          value={filterValue[0]?.toString() ?? ''}
          onChange={(val) => {
            const min = val === '' ? undefined : Number(val)
            column.setFilterValue([min, filterValue[1]])
          }}
          className="h-7 text-xs"
          placeholder={t('tables:filter.minValue')}
        />
      </div>
      <div className="space-y-1">
        <label className="text-[10px] font-medium text-muted-foreground">{t('tables:filter.max')}</label>
        <DebouncedInput
          type="number"
          value={filterValue[1]?.toString() ?? ''}
          onChange={(val) => {
            const max = val === '' ? undefined : Number(val)
            column.setFilterValue([filterValue[0], max])
          }}
          className="h-7 text-xs"
          placeholder={t('tables:filter.maxValue')}
        />
      </div>
      {hasFilter && (
        <Button
          variant="ghost"
          size="sm"
          className="h-6 w-full text-xs"
          onClick={() => column.setFilterValue(undefined)}
        >
          {t('tables:filter.clear')}
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
