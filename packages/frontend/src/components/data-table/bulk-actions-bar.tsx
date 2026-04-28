import type { Table } from '@tanstack/react-table'
import type { LucideIcon } from 'lucide-react'
import { X } from 'lucide-react'
import { Portal } from 'radix-ui'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { cn } from '~/lib/utils'

export interface BulkAction<TData> {
  label: string
  icon?: LucideIcon
  onClick: (selectedRows: TData[]) => void
  variant?: 'default' | 'destructive'
}

interface BulkActionsBarProps<TData> {
  table: Table<TData>
  actions: BulkAction<TData>[]
}

export function BulkActionsBar<TData>({
  table,
  actions,
}: BulkActionsBarProps<TData>) {
  const { t } = useTranslation('tables')

  const selectedRows = table.getFilteredSelectedRowModel().rows
  const count = selectedRows.length

  if (count === 0)
    return null

  return (
    <Portal.Root>
      <div
        className={cn(
          'fixed bottom-4 left-1/2 z-50 -translate-x-1/2',
          'flex items-center gap-3 rounded-lg border bg-background/95 px-4 py-2 shadow-lg backdrop-blur',
          'animate-in fade-in-0 slide-in-from-bottom-2',
          'transition-all duration-200 ease-out',
        )}
      >
        {/* Selected count */}
        <span className="text-sm font-medium tabular-nums">
          {t('tables:selected', { count })}
        </span>

        {/* Action buttons */}
        <div className="flex items-center gap-1">
          {actions.map(action => (
            <Button
              key={action.label}
              variant={action.variant === 'destructive' ? 'destructive' : 'secondary'}
              size="sm"
              onClick={() =>
                action.onClick(selectedRows.map(row => row.original))}
            >
              {action.icon && <action.icon className="size-4" />}
              {action.label}
            </Button>
          ))}
        </div>

        {/* Deselect all */}
        <Button
          variant="ghost"
          size="icon-sm"
          onClick={() => table.toggleAllRowsSelected(false)}
          aria-label={t('tables:deselectAll')}
        >
          <X className="size-4" />
        </Button>
      </div>
    </Portal.Root>
  )
}
