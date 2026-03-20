import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import type { StorageResponse } from '~/generated/types'
import { Archive, Pencil, Trash2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useStorages } from './storages-provider'

interface DataTableRowActionsProps {
  row: Row<StorageResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useStorages()

  const actions: RowAction[] = [
    {
      label: t('common:actions.edit'),
      icon: Pencil,
      inline: true,
      onClick: () => {
        setCurrentRow(row.original)
        setOpen('update')
      },
    },
    {
      label: t('common:actions.softDelete'),
      icon: Archive,
      onClick: () => {
        setCurrentRow(row.original)
        setOpen('delete')
      },
    },
    {
      label: t('common:actions.hardDelete'),
      icon: Trash2,
      variant: 'destructive',
      onClick: () => {
        setCurrentRow(row.original)
        setOpen('hard-delete')
      },
    },
  ]

  return <RowActions actions={actions} />
}
