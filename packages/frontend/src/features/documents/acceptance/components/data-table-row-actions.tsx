import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import type { AcceptanceResponse } from '~/generated/types'
import { Archive, Pencil, PlayCircle, RotateCcw, Trash2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useAcceptance } from './acceptance-provider'

interface DataTableRowActionsProps {
  row: Row<AcceptanceResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useAcceptance()

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
    ...(row.original.status === 'DRAFT'
      ? [{
          label: t('common:actions.execute'),
          icon: PlayCircle,
          onClick: () => {
            setCurrentRow(row.original)
            setOpen('execute')
          },
        }]
      : []),
    ...(row.original.status === 'POSTED'
      ? [{
          label: t('common:actions.revert'),
          icon: RotateCcw,
          onClick: () => {
            setCurrentRow(row.original)
            setOpen('revert')
          },
        }]
      : []),
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
