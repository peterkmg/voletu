import type { Row } from '@tanstack/react-table'
import type { EntityLifecycleAction } from './entity-dialog-state'
import type { RowAction } from '~/components/data-table'
import { useNavigate } from '@tanstack/react-router'
import { Archive, ArrowUpRight, FilePlus2, Pencil, Play, Trash2, Undo2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useAuthStore } from '~/stores/auth-store'

interface CreateRowActionsConfig<TRow extends { id: string }> {
  useEntity: () => {
    openUpdate: (row: TRow) => void
    openDelete: (row: TRow, mode?: 'soft' | 'hard') => void
    openLifecycle: (row: TRow, action: EntityLifecycleAction) => void
    openIssueAcceptance?: (row: TRow) => void
  }
  lifecycle?: boolean
  deleteOnly?: boolean
  disableEdit?: boolean
  getDetailPath?: (row: TRow) => string
  pipelineActions?: {
    editVisible?: (row: TRow) => boolean
    issueAcceptance?: { visible: (row: TRow) => boolean }
  }
}

export function createRowActions<TRow extends { id: string }>(
  config: CreateRowActionsConfig<TRow>,
) {
  return function DataTableRowActions({ row }: { row: Row<TRow> }) {
    const { t } = useTranslation(['common', 'documents'])
    const navigate = useNavigate()
    const { openUpdate, openDelete, openLifecycle, openIssueAcceptance } = config.useEntity()
    const userRole = useAuthStore(s => s.user?.role)

    const selectUpdate = () => openUpdate(row.original)
    const selectDelete = (mode?: 'soft' | 'hard') => () => openDelete(row.original, mode)
    const selectLifecycle = (action: EntityLifecycleAction) => () =>
      openLifecycle(row.original, action)
    const selectIssueAcceptance = () => openIssueAcceptance?.(row.original)

    const actions: RowAction[] = []

    if (config.getDetailPath) {
      actions.push({
        label: t('common:actions.viewDetails'),
        icon: ArrowUpRight,
        inline: true,
        onClick: () => navigate({ to: config.getDetailPath!(row.original) }),
      })
    }

    const editVisible = config.pipelineActions?.editVisible
      ? config.pipelineActions.editVisible(row.original)
      : !config.disableEdit
    if (!config.deleteOnly && editVisible) {
      actions.push({
        label: t('common:actions.edit'),
        icon: Pencil,
        inline: true,
        onClick: selectUpdate,
      })
    }

    if (config.pipelineActions?.issueAcceptance?.visible(row.original)) {
      actions.push({
        label: t('documents:actions.issueAcceptance'),
        icon: FilePlus2,
        inline: true,
        onClick: selectIssueAcceptance,
      })
    }

    if (config.lifecycle) {
      const status = (row.original as { status?: string }).status

      if (status === 'DRAFT') {
        actions.push({
          label: t('documents:lifecycle.execute'),
          icon: Play,
          onClick: selectLifecycle('execute'),
        })
      }

      if (
        status === 'EXECUTED'
        && (userRole === 'ADMIN' || userRole === 'SENIOR_SUPERVISOR')
      ) {
        actions.push({
          label: t('documents:lifecycle.revert'),
          icon: Undo2,
          onClick: selectLifecycle('revert'),
        })
      }
    }

    if (!config.disableEdit) {
      actions.push({
        label: t('common:actions.softDelete'),
        icon: Archive,
        onClick: selectDelete(),
      })
    }

    if (config.deleteOnly) {
      actions.push({
        label: t('common:actions.delete'),
        icon: Trash2,
        variant: 'destructive' as const,
        onClick: selectDelete(),
      })
    }

    if (userRole === 'ADMIN' && !config.disableEdit) {
      actions.push({
        label: t('common:actions.hardDelete'),
        icon: Trash2,
        variant: 'destructive' as const,
        onClick: selectDelete('hard'),
      })
    }

    return <RowActions actions={actions} />
  }
}
