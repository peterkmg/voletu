import type { ColumnDef } from '@tanstack/react-table'
import { getCoreRowModel, useReactTable } from '@tanstack/react-table'
import { useTranslation } from 'react-i18next'
import { DataTable } from '~/components/data-table/data-table'
import { Button } from '~/components/ui/button'
import { Card, CardAction, CardContent, CardHeader, CardTitle } from '~/components/ui/card'

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
    <Card>
      <CardHeader>
        <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
          {sectionTitle}
        </CardTitle>
        {!isLocked && onAddItem && (
          <CardAction>
            <Button variant="outline" size="sm" onClick={onAddItem}>
              {t('actions.addItem', 'Add Item')}
            </Button>
          </CardAction>
        )}
      </CardHeader>
      <CardContent>
        <DataTable table={table} columns={columns} mode="paginated" />
      </CardContent>
    </Card>
  )
}
