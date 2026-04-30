import type { Table } from '@tanstack/react-table'
import type { TableMode } from './table-mode-toggle'
import { X } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { DebouncedInput } from '~/components/ui/debounced-input'
import { DensityToggle } from './density'
import { ExportButton } from './export-button'
import { TableModeToggle } from './table-mode-toggle'
import { DataTableViewOptions } from './view-options'

interface DataTableToolbarProps<TData> {
  table: Table<TData>
  searchPlaceholder?: string
  searchKey?: string
  tableMode?: TableMode
  onTableModeChange?: (mode: TableMode) => void
  exportFilename?: string
  /** Optional action buttons (e.g. Create) rendered in the toolbar's right section. */
  actions?: React.ReactNode
}

export function DataTableToolbar<TData>({
  table,
  searchPlaceholder,
  searchKey,
  tableMode,
  onTableModeChange,
  exportFilename,
  actions,
}: DataTableToolbarProps<TData>) {
  const { t } = useTranslation('tables')
  const isFiltered
    = table.getState().columnFilters.length > 0 || table.getState().globalFilter

  const placeholder = searchPlaceholder ?? t('tables:filter.search')

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
        {isFiltered && (
          <Button
            variant="ghost"
            onClick={() => {
              table.resetColumnFilters()
              table.setGlobalFilter('')
            }}
            className="h-8 px-2 lg:px-3"
          >
            {t('tables:filter.reset')}
            <X className="ms-2 h-4 w-4" />
          </Button>
        )}
      </div>
      <div className="flex items-center gap-2">
        <ExportButton table={table} filename={exportFilename} />
        {tableMode && onTableModeChange && (
          <TableModeToggle mode={tableMode} onModeChange={onTableModeChange} />
        )}
        <DensityToggle />
        <DataTableViewOptions table={table} />
        {actions}
      </div>
    </div>
  )
}
