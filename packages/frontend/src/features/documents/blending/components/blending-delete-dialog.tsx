import type { BlendingResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { blendingDocumentHardDelete, blendingDocumentSoftDelete } from '~/generated/client'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'

interface BlendingDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: BlendingResponse | null
  variant: 'soft' | 'hard'
}

export function BlendingDeleteDialog({ open, onOpenChange, currentRow, variant }: BlendingDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={blendingDocumentHardDelete}
      softDeleteFn={blendingDocumentSoftDelete}
      queryKey={blendingDocumentListQueryKey()}
      entityLabel={t('documents:blending.singular')}
    />
  )
}
