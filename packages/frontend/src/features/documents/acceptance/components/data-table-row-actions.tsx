import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import type { AcceptanceResponse } from '~/generated/types'
import { Archive, Pencil, Play, Trash2, Undo2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useAuthStore } from '~/stores/auth-store'
import { useAcceptance } from './acceptance-provider'

interface DataTableRowActionsProps {
  row: Row<AcceptanceResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useAcceptance()
  const userRole = useAuthStore(s => s.auth.user?.role)
  const status = row.original.status

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
    ...(status === 'DRAFT'
      ? [{
          label: t('common:actions.execute'),
          icon: Play,
          onClick: () => {
            setCurrentRow(row.original)
            setOpen('execute')
          },
        }]
      : []),
    ...(status === 'POSTED' && (userRole === 'ADMIN' || userRole === 'SENIOR_SUPERVISOR')
      ? [{
          label: t('common:actions.revert'),
          icon: Undo2,
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
    ...(userRole === 'ADMIN'
      ? [{
          label: t('common:actions.hardDelete'),
          icon: Trash2,
          variant: 'destructive' as const,
          onClick: () => {
            setCurrentRow(row.original)
            setOpen('hard-delete')
          },
        }]
      : []),
  ]

  return <RowActions actions={actions} />
}
