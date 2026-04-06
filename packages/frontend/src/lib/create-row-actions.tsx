import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import { useNavigate } from '@tanstack/react-router'
import { Archive, Pencil, Play, Search, Trash2, Undo2 } from 'lucide-react'
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
  /** Hide edit button (for pipeline tables without update dialog) */
  disableEdit?: boolean
  /** Returns the detail page path for the given row. Adds an inline details button. */
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

    const actions: RowAction[] = []

    // Slot 1: Details (navigate to detail page)
    if (config.getDetailPath) {
      actions.push({
        label: t('common:actions.viewDetails'),
        icon: Search,
        inline: true,
        onClick: () => navigate({ to: config.getDetailPath!(row.original) }),
      })
    }

    // Slot 2: Edit (open edit dialog)
    if (!config.deleteOnly && !config.disableEdit) {
      actions.push({
        label: t('common:actions.edit'),
        icon: Pencil,
        inline: true,
        onClick: select('update'),
      })
    }

    // Slot 3: Overflow menu actions
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
        status === 'EXECUTED'
        && (userRole === 'ADMIN' || userRole === 'SENIOR_SUPERVISOR')
      ) {
        actions.push({
          label: t('common:actions.revert'),
          icon: Undo2,
          onClick: select('revert'),
        })
      }
    }

    if (!config.disableEdit) {
      actions.push({
        label: t('common:actions.softDelete'),
        icon: Archive,
        onClick: select('delete'),
      })
    }

    if (config.deleteOnly) {
      actions.push({
        label: t('common:actions.delete'),
        icon: Trash2,
        variant: 'destructive' as const,
        onClick: select('delete'),
      })
    }

    if (userRole === 'ADMIN' && !config.disableEdit) {
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
