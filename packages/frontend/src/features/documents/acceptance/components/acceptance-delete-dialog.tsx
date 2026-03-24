import type { AcceptanceResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { acceptanceDocumentHardDelete, acceptanceDocumentSoftDelete } from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'

interface AcceptanceDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: AcceptanceResponse | null
  variant: 'soft' | 'hard'
}

export function AcceptanceDeleteDialog({ open, onOpenChange, currentRow, variant }: AcceptanceDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={acceptanceDocumentHardDelete}
      softDeleteFn={acceptanceDocumentSoftDelete}
      queryKey={acceptanceDocumentListQueryKey()}
      entityLabel={t('documents:acceptance.singular')}
    />
  )
}
