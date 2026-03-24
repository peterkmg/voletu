import type { DispatchResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { dispatchDocumentHardDelete, dispatchDocumentSoftDelete } from '~/generated/client'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'

interface DispatchDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: DispatchResponse | null
  variant: 'soft' | 'hard'
}

export function DispatchDeleteDialog({ open, onOpenChange, currentRow, variant }: DispatchDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={dispatchDocumentHardDelete}
      softDeleteFn={dispatchDocumentSoftDelete}
      queryKey={dispatchDocumentListQueryKey()}
      entityLabel={t('documents:dispatch.singular')}
    />
  )
}
