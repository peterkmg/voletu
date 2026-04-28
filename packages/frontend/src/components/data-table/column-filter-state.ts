import type { Column } from '@tanstack/react-table'

export interface DayNode { date: string, count: number }
export interface MonthNode { year: number, month: number, label: string, days: DayNode[], totalCount: number }
export interface YearNode { year: number, months: MonthNode[], totalCount: number }

export function selectAllState(allValues: string[], selectedSet: Set<string>): boolean | 'indeterminate' {
  if (allValues.length === 0)
    return false

  const count = allValues.filter(v => selectedSet.has(v)).length
  if (count === 0)
    return false

  return count === allValues.length ? true : 'indeterminate'
}

export function getSelectedSet(filterValue: unknown, allValues: string[]): Set<string> {
  if (filterValue === undefined || filterValue === null)
    return new Set(allValues)

  return new Set(filterValue as string[])
}

export function commitFilter<TData, TValue>(
  column: Pick<Column<TData, TValue>, 'setFilterValue'>,
  next: Set<string>,
  allValues: string[],
) {
  if (allValues.length > 0 && allValues.every(v => next.has(v)))
    column.setFilterValue(undefined)
  else
    column.setFilterValue(Array.from(next))
}

export function buildDateTree(facets: Map<unknown, number>): { years: YearNode[], allDates: string[] } {
  const dayMap = new Map<string, number>()
  for (const [value, count] of facets) {
    if (value == null)
      continue

    const date = String(value).slice(0, 10)
    dayMap.set(date, (dayMap.get(date) ?? 0) + count)
  }

  const yearMap = new Map<number, Map<number, DayNode[]>>()
  for (const [date, count] of dayMap) {
    const [yearValue, monthValue] = date.split('-')
    const year = Number(yearValue)
    const month = Number(monthValue)
    const monthMap = yearMap.get(year) ?? new Map<number, DayNode[]>()
    const days = monthMap.get(month) ?? []

    days.push({ date, count })
    monthMap.set(month, days)
    yearMap.set(year, monthMap)
  }

  const allDates: string[] = []
  const years: YearNode[] = []

  for (const [year, monthMap] of yearMap) {
    const months: MonthNode[] = []
    let yearTotal = 0

    for (const [month, days] of monthMap) {
      days.sort((a, b) => b.date.localeCompare(a.date))
      const totalCount = days.reduce((sum, day) => sum + day.count, 0)

      yearTotal += totalCount
      months.push({
        year,
        month,
        label: new Date(year, month - 1).toLocaleString('default', { month: 'short' }),
        days,
        totalCount,
      })
      allDates.push(...days.map(day => day.date))
    }

    months.sort((a, b) => b.month - a.month)
    years.push({ year, months, totalCount: yearTotal })
  }

  years.sort((a, b) => b.year - a.year)
  return { years, allDates }
}
