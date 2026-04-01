import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import { Archive, Pencil, Play, Trash2, Undo2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useAuthStore } from '~/stores/auth-store'

interface CreateRowActionsConfig<TRow extends { id: string }> {
  useEntity: () => {
    setOpen: (s: never) => void
    setCurrentRow: (row: TRow | null) => void
  }
  lifecycle?: boolean
  deleteOnly?: boolean
}

export function createRowActions<TRow extends { id: string }>(
  config: CreateRowActionsConfig<TRow>,
) {
  return function DataTableRowActions({ row }: { row: Row<TRow> }) {
    const { t } = useTranslation(['common'])
    const { setOpen, setCurrentRow } = config.useEntity()
    const userRole = useAuthStore(s => s.user?.role)

    const select = (dialogType: string) => () => {
      setCurrentRow(row.original)
      ;(setOpen as (s: string | null) => void)(dialogType)
    }

    if (config.deleteOnly) {
      return (
        <RowActions
          actions={[
            {
              label: t('common:actions.delete'),
              icon: Trash2,
              variant: 'destructive' as const,
              onClick: select('delete'),
            },
          ]}
        />
      )
    }

    const actions: RowAction[] = [
      {
        label: t('common:actions.edit'),
        icon: Pencil,
        inline: true,
        onClick: select('update'),
      },
    ]

    if (config.lifecycle) {
      const status = (row.original as { status?: string }).status
      if (status === 'DRAFT') {
        actions.push({
          label: t('common:actions.execute'),
          icon: Play,
          onClick: select('execute'),
        })
      }
      if (
        status === 'POSTED'
        && (userRole === 'ADMIN' || userRole === 'SENIOR_SUPERVISOR')
      ) {
        actions.push({
          label: t('common:actions.revert'),
          icon: Undo2,
          onClick: select('revert'),
        })
      }
    }

    actions.push({
      label: t('common:actions.softDelete'),
      icon: Archive,
      onClick: select('delete'),
    })

    if (userRole === 'ADMIN') {
      actions.push({
        label: t('common:actions.hardDelete'),
        icon: Trash2,
        variant: 'destructive' as const,
        onClick: select('hard-delete'),
      })
    }

    return <RowActions actions={actions} />
  }
}
