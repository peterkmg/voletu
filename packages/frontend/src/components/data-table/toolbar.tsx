import type { Table } from '@tanstack/react-table'
import type { TableMode } from './table-mode-toggle'
import { X } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { DebouncedInput } from './debounced-input'
import { DensityToggle } from './density-toggle'
import { ExportButton } from './export-button'
import { DataTableFacetedFilter } from './faceted-filter'
import { TableModeToggle } from './table-mode-toggle'
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
  tableMode?: TableMode
  onTableModeChange?: (mode: TableMode) => void
}

export function DataTableToolbar<TData>({
  table,
  searchPlaceholder,
  searchKey,
  filters = [],
  tableMode,
  onTableModeChange,
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
              <DebouncedInput
                placeholder={placeholder}
                value={
                  (table.getColumn(searchKey)?.getFilterValue() as string) ?? ''
                }
                onChange={value =>
                  table.getColumn(searchKey)?.setFilterValue(value)}
                className="h-8 w-[150px] lg:w-[250px]"
              />
            )
          : (
              <DebouncedInput
                placeholder={placeholder}
                value={table.getState().globalFilter ?? ''}
                onChange={value => table.setGlobalFilter(value)}
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
        <ExportButton table={table} />
        {tableMode && onTableModeChange && (
          <TableModeToggle mode={tableMode} onModeChange={onTableModeChange} />
        )}
        <DensityToggle />
        <DataTableViewOptions table={table} />
      </div>
    </div>
  )
}
