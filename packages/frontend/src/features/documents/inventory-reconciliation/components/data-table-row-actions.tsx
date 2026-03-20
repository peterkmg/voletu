import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import type { InventoryReconciliationResponse } from '~/generated/types'
import { Archive, Pencil, Play, Trash2, Undo2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useReconciliation } from './reconciliation-provider'

interface DataTableRowActionsProps {
  row: Row<InventoryReconciliationResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useReconciliation()

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
      label: t('common:actions.execute'),
      icon: Play,
      onClick: () => {
        setCurrentRow(row.original)
        setOpen('execute')
      },
    },
    {
      label: t('common:actions.revert'),
      icon: Undo2,
      onClick: () => {
        setCurrentRow(row.original)
        setOpen('revert')
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
