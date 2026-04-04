import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import { useNavigate } from '@tanstack/react-router'
import { Archive, ChevronRight, Pencil, Play, Trash2, Undo2 } from 'lucide-react'
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
  /** Returns the detail page path for the given row. Adds an inline navigate button. */
  getDetailPath?: (row: TRow) => string
}

export function createRowActions<TRow extends { id: string }>(
  config: CreateRowActionsConfig<TRow>,
) {
  return function DataTableRowActions({ row }: { row: Row<TRow> }) {
    const { t } = useTranslation(['common'])
    const navigate = useNavigate()
    const { setOpen, setCurrentRow } = config.useEntity()
    const userRole = useAuthStore(s => s.user?.role)

    const select = (dialogType: string) => () => {
      setCurrentRow(row.original)
      ;(setOpen as (s: string | null) => void)(dialogType)
    }

    const navAction: RowAction | undefined = config.getDetailPath
      ? {
          label: t('common:actions.viewDetails'),
          icon: ChevronRight,
          inline: true,
          onClick: () => navigate({ to: config.getDetailPath!(row.original) }),
        }
      : undefined

    if (config.deleteOnly) {
      const actions: RowAction[] = []
      if (navAction)
        actions.push(navAction)
      actions.push({
        label: t('common:actions.delete'),
        icon: Trash2,
        variant: 'destructive' as const,
        onClick: select('delete'),
      })
      return <RowActions actions={actions} />
    }

    const actions: RowAction[] = [
      {
        label: t('common:actions.edit'),
        icon: Pencil,
        inline: true,
        onClick: select('update'),
      },
    ]

    // Inline navigate button (after edit)
    if (navAction)
      actions.push(navAction)

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
