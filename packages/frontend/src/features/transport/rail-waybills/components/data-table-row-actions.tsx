import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import type { RailWaybillResponse } from '~/generated/types'
import { Archive, Pencil, Trash2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useAuthStore } from '~/stores/auth-store'
import { useRailWaybills } from './rail-waybills-provider'

interface DataTableRowActionsProps {
  row: Row<RailWaybillResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useRailWaybills()
  const userRole = useAuthStore(s => s.auth.user?.role)

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
