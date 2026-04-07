import type { Column } from '@tanstack/react-table'
import {
  ArrowDown,
  ArrowUp,
  ChevronsUpDown,
  Columns3,
  EyeOff,
  Filter,
  PinIcon,
  PinOff,
} from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { cn } from '~/lib/utils'
import { ColumnFilterInline } from './column-filter'

type DataTableColumnHeaderProps<TData, TValue>
  = React.HTMLAttributes<HTMLDivElement> & {
    column: Column<TData, TValue>
    title: string
  }

/** Checks whether a column has an active filter value. */
function hasActiveFilter<TData, TValue>(column: Column<TData, TValue>): boolean {
  const val = column.getFilterValue()
  if (val == null)
    return false
  if (Array.isArray(val))
    return val.length > 0
  return true
}

export function DataTableColumnHeader<TData, TValue>({
  column,
  title,
  className,
}: DataTableColumnHeaderProps<TData, TValue>) {
  const align = column.columnDef.meta?.align
  const justifyCls = align === 'right' ? 'justify-end' : align === 'center' ? 'justify-center' : ''

  const canSort = column.getCanSort()
  const canFilter = column.columnDef.meta?.enableHeaderFilter !== false && column.getCanFilter()
  const canPin = column.getCanPin()
  const canHide = column.getCanHide()
  const hasInteraction = canSort || canFilter || canPin || canHide

  // No interactive features — plain title
  if (!hasInteraction) {
    return (
      <div className={cn('flex items-center gap-1', justifyCls, className)}>
        <span className="truncate">{title}</span>
      </div>
    )
  }

  const sorted = column.getIsSorted()
  const filterActive = canFilter && hasActiveFilter(column)

  // Determine trigger icon based on state
  let TriggerIcon = canSort ? ChevronsUpDown : Filter
  if (sorted === 'asc')
    TriggerIcon = ArrowUp
  if (sorted === 'desc')
    TriggerIcon = ArrowDown

  const triggerButton = (
    <DropdownMenuTrigger asChild>
      <Button
        variant="ghost"
        size="icon"
        className={cn(
          'relative h-7 w-7 shrink-0 data-[state=open]:bg-accent',
          (sorted || filterActive) && 'text-foreground',
        )}
      >
        <TriggerIcon className="size-3.5" />
        {filterActive && (
          <span className="absolute -right-0.5 -top-0.5 size-2 rounded-full bg-primary" />
        )}
      </Button>
    </DropdownMenuTrigger>
  )

  return (
    <div className={cn('flex items-center gap-0.5', justifyCls, className)}>
      {align === 'right' && (
        <DropdownMenu modal={false}>
          {triggerButton}
          <HeaderDropdownContent
            column={column}
            canSort={canSort}
            canFilter={canFilter}
            filterActive={filterActive}
            canPin={canPin}
            canHide={canHide}
          />
        </DropdownMenu>
      )}
      <span className="truncate">{title}</span>
      {align !== 'right' && (
        <DropdownMenu modal={false}>
          {triggerButton}
          <HeaderDropdownContent
            column={column}
            canSort={canSort}
            canFilter={canFilter}
            filterActive={filterActive}
            canPin={canPin}
            canHide={canHide}
          />
        </DropdownMenu>
      )}
    </div>
  )
}

/** Compact dropdown with sort items + filter/column submenus. */
function HeaderDropdownContent<TData, TValue>({
  column,
  canSort,
  canFilter,
  filterActive,
  canPin,
  canHide,
}: {
  column: Column<TData, TValue>
  canSort: boolean
  canFilter: boolean
  filterActive: boolean
  canPin: boolean
  canHide: boolean
}) {
  const hasColumnMgmt = canPin || canHide
  const { t } = useTranslation('common')

  return (
    <DropdownMenuContent align="start" className="w-40 text-xs">
      {/* --- Sort items (always top-level for quick access) --- */}
      {canSort && (
        <>
          <DropdownMenuItem onSelect={() => column.toggleSorting(false)}>
            <ArrowUp className="size-3.5 text-muted-foreground/70" />
            {t('dataTable.ascending')}
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => column.toggleSorting(true)}>
            <ArrowDown className="size-3.5 text-muted-foreground/70" />
            {t('dataTable.descending')}
          </DropdownMenuItem>
          {(canFilter || hasColumnMgmt) && <DropdownMenuSeparator />}
        </>
      )}

      {/* --- Filter submenu --- */}
      {canFilter && (
        <>
          <DropdownMenuSub>
            <DropdownMenuSubTrigger>
              <Filter className="size-3.5 text-muted-foreground/70" />
              {t('dataTable.filter')}
              {filterActive && (
                <span className="ms-auto size-2 rounded-full bg-primary" />
              )}
            </DropdownMenuSubTrigger>
            <DropdownMenuSubContent className="w-52 p-0 text-xs">
              {/* Stop propagation prevents DropdownMenu from stealing keyboard
                  events that cmdk needs for search input and list navigation. */}
              <div onKeyDown={e => e.stopPropagation()}>
                <ColumnFilterInline column={column} />
              </div>
            </DropdownMenuSubContent>
          </DropdownMenuSub>
          {hasColumnMgmt && <DropdownMenuSeparator />}
        </>
      )}

      {/* --- Column management submenu --- */}
      {hasColumnMgmt && (
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <Columns3 className="size-3.5 text-muted-foreground/70" />
            {t('dataTable.column')}
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            {canPin && (
              column.getIsPinned()
                ? (
                    <DropdownMenuItem onSelect={() => column.pin(false)}>
                      <PinOff className="size-3.5 text-muted-foreground/70" />
                      {t('dataTable.unpin')}
                    </DropdownMenuItem>
                  )
                : (
                    <>
                      <DropdownMenuItem onSelect={() => column.pin('left')}>
                        <PinIcon className="size-3.5 text-muted-foreground/70" />
                        {t('dataTable.pinLeft')}
                      </DropdownMenuItem>
                      <DropdownMenuItem onSelect={() => column.pin('right')}>
                        <PinIcon className="size-3.5 rotate-90 text-muted-foreground/70" />
                        {t('dataTable.pinRight')}
                      </DropdownMenuItem>
                    </>
                  )
            )}
            {canHide && (
              <DropdownMenuItem onSelect={() => column.toggleVisibility(false)}>
                <EyeOff className="size-3.5 text-muted-foreground/70" />
                {t('dataTable.hide')}
              </DropdownMenuItem>
            )}
          </DropdownMenuSubContent>
        </DropdownMenuSub>
      )}
    </DropdownMenuContent>
  )
}
