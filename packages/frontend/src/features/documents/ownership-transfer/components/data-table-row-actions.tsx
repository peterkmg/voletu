import type { Row } from '@tanstack/react-table'
import type { RowAction } from '~/components/data-table'
import type { OwnershipTransferResponse } from '~/generated/types'
import { Play, Undo2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { RowActions } from '~/components/data-table'
import { useOwnershipTransfer } from './ownership-transfer-provider'

interface DataTableRowActionsProps {
  row: Row<OwnershipTransferResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useOwnershipTransfer()

  const actions: RowAction[] = [
    ...(row.original.status === 'DRAFT'
      ? [{
          label: t('common:actions.execute'),
          icon: Play,
          inline: true,
          onClick: () => {
            setCurrentRow(row.original)
            setOpen('execute')
          },
        }]
      : []),
    ...(row.original.status === 'POSTED'
      ? [{
          label: t('common:actions.revert'),
          icon: Undo2,
          inline: true,
          onClick: () => {
            setCurrentRow(row.original)
            setOpen('revert')
          },
        }]
      : []),
  ]

  return <RowActions actions={actions} />
}
