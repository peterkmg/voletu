import type { Row, Table } from '@tanstack/react-table'
import { useTranslation } from 'react-i18next'
import { Checkbox } from '~/components/ui/checkbox'

export function SelectAllCheckbox<T>({ table }: { table: Table<T> }) {
  const { t } = useTranslation('tables')

  return (
    <Checkbox
      checked={
        table.getIsAllPageRowsSelected()
        || (table.getIsSomePageRowsSelected() && 'indeterminate')
      }
      onCheckedChange={value => table.toggleAllPageRowsSelected(!!value)}
      aria-label={t('tables:selectAll')}
      className="translate-y-[2px]"
    />
  )
}

export function SelectRowCheckbox<T>({ row }: { row: Row<T> }) {
  const { t } = useTranslation('tables')

  return (
    <Checkbox
      checked={row.getIsSelected()}
      onCheckedChange={value => row.toggleSelected(!!value)}
      aria-label={t('tables:selectRow')}
      className="translate-y-[2px]"
    />
  )
}
