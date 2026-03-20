import type { Table } from '@tanstack/react-table'
import { X } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { Input } from '~/components/ui/input'
import { DensityToggle } from './density-toggle'
import { DataTableFacetedFilter } from './faceted-filter'
import { DataTableViewOptions } from './view-options'

interface DataTableToolbarProps<TData> {
  table: Table<TData>
  searchPlaceholder?: string
  searchKey?: string
  filters?: {
    columnId: string
    title: string
    options: {
      label: string
      value: string
      icon?: React.ComponentType<{ className?: string }>
    }[]
  }[]
}

export function DataTableToolbar<TData>({
  table,
  searchPlaceholder,
  searchKey,
  filters = [],
}: DataTableToolbarProps<TData>) {
  const { t } = useTranslation('common')
  const isFiltered
    = table.getState().columnFilters.length > 0 || table.getState().globalFilter

  const placeholder = searchPlaceholder ?? `${t('actions.search')}...`

  return (
    <div className="flex items-center justify-between">
      <div className="flex flex-1 flex-col-reverse items-start gap-y-2 sm:flex-row sm:items-center sm:space-x-2">
        {searchKey
          ? (
              <Input
                placeholder={placeholder}
                value={
                  (table.getColumn(searchKey)?.getFilterValue() as string) ?? ''
                }
                onChange={event =>
                  table.getColumn(searchKey)?.setFilterValue(event.target.value)}
                className="h-8 w-[150px] lg:w-[250px]"
              />
            )
          : (
              <Input
                placeholder={placeholder}
                value={table.getState().globalFilter ?? ''}
                onChange={event => table.setGlobalFilter(event.target.value)}
                className="h-8 w-[150px] lg:w-[250px]"
              />
            )}
        <div className="flex gap-x-2">
          {filters.map((filter) => {
            const column = table.getColumn(filter.columnId)
            if (!column)
              return null
            return (
              <DataTableFacetedFilter
                key={filter.columnId}
                column={column}
                title={filter.title}
                options={filter.options}
              />
            )
          })}
        </div>
        {isFiltered && (
          <Button
            variant="ghost"
            onClick={() => {
              table.resetColumnFilters()
              table.setGlobalFilter('')
            }}
            className="h-8 px-2 lg:px-3"
          >
            {t('actions.reset')}
            <X className="ms-2 h-4 w-4" />
          </Button>
        )}
      </div>
      <div className="flex items-center gap-2">
        <DensityToggle />
        <DataTableViewOptions table={table} />
      </div>
    </div>
  )
}
