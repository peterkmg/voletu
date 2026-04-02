import type { ColumnDef } from '@tanstack/react-table'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { DataTable } from '~/components/data-table/data-table'
import { getCoreRowModel, useReactTable } from '@tanstack/react-table'

interface ChildItemsTableProps<TItem> {
  items: TItem[]
  columns: ColumnDef<TItem>[]
  isLocked: boolean
  onAddItem?: () => void
  sectionTitle: string
}

export function ChildItemsTable<TItem>({
  items,
  columns,
  isLocked,
  onAddItem,
  sectionTitle,
}: ChildItemsTableProps<TItem>) {
  const { t } = useTranslation('common')

  const table = useReactTable({
    data: items,
    columns,
    getCoreRowModel: getCoreRowModel(),
  })

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
          {sectionTitle}
        </h3>
        {!isLocked && onAddItem && (
          <Button variant="outline" size="sm" onClick={onAddItem}>
            {t('actions.addItem', 'Add Item')}
          </Button>
        )}
      </div>
      <DataTable table={table} columns={columns} />
    </div>
  )
}
