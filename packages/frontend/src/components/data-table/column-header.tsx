import type { Column } from '@tanstack/react-table'
import { ArrowDown, ArrowUp, ChevronsUpDown, EyeOff, PinIcon, PinOff } from 'lucide-react'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { cn } from '~/lib/utils'
import { ColumnFilter } from './column-filter'

type DataTableColumnHeaderProps<TData, TValue>
  = React.HTMLAttributes<HTMLDivElement> & {
    column: Column<TData, TValue>
    title: string
  }

export function DataTableColumnHeader<TData, TValue>({
  column,
  title,
  className,
}: DataTableColumnHeaderProps<TData, TValue>) {
  const align = column.columnDef.meta?.align
  const justifyCls = align === 'right' ? 'justify-end' : align === 'center' ? 'justify-center' : ''

  if (!column.getCanSort()) {
    return (
      <div className={cn('flex items-center gap-1', justifyCls, className)}>
        {title}
        <ColumnFilter column={column} />
      </div>
    )
  }

  return (
    <div className={cn('flex items-center', justifyCls, className)}>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            size="sm"
            className={cn(
              'h-8 data-[state=open]:bg-accent',
              align === 'right' ? '-me-3' : '-ms-3',
            )}
          >
            <span>{title}</span>
            {column.getIsSorted() === 'desc'
              ? (
                  <ArrowDown className="ms-2 h-4 w-4" />
                )
              : column.getIsSorted() === 'asc'
                ? (
                    <ArrowUp className="ms-2 h-4 w-4" />
                  )
                : (
                    <ChevronsUpDown className="ms-2 h-4 w-4" />
                  )}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          <DropdownMenuItem onSelect={() => column.toggleSorting(false)}>
            <ArrowUp className="size-3.5 text-muted-foreground/70" />
            Asc
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => column.toggleSorting(true)}>
            <ArrowDown className="size-3.5 text-muted-foreground/70" />
            Desc
          </DropdownMenuItem>
          {(column.getCanHide() || column.getCanPin()) && (
            <>
              <DropdownMenuSeparator />
              {column.getCanPin() && (
                column.getIsPinned()
                  ? (
                      <DropdownMenuItem onSelect={() => column.pin(false)}>
                        <PinOff className="size-3.5 text-muted-foreground/70" />
                        Unpin
                      </DropdownMenuItem>
                    )
                  : (
                      <>
                        <DropdownMenuItem onSelect={() => column.pin('left')}>
                          <PinIcon className="size-3.5 text-muted-foreground/70" />
                          Pin left
                        </DropdownMenuItem>
                        <DropdownMenuItem onSelect={() => column.pin('right')}>
                          <PinIcon className="size-3.5 rotate-90 text-muted-foreground/70" />
                          Pin right
                        </DropdownMenuItem>
                      </>
                    )
              )}
              {column.getCanHide() && (
                <DropdownMenuItem onSelect={() => column.toggleVisibility(false)}>
                  <EyeOff className="size-3.5 text-muted-foreground/70" />
                  Hide
                </DropdownMenuItem>
              )}
            </>
          )}
        </DropdownMenuContent>
      </DropdownMenu>
      <ColumnFilter column={column} />
    </div>
  )
}
