import type { RailWaybillResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { transportRailWaybillHardDelete, transportRailWaybillSoftDelete } from '~/generated/client'
import { transportRailWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'

interface RailWaybillDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: RailWaybillResponse | null
  variant: 'soft' | 'hard'
}

export function RailWaybillDeleteDialog({ open, onOpenChange, currentRow, variant }: RailWaybillDeleteDialogProps) {
  const { t } = useTranslation(['common', 'transport'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={transportRailWaybillHardDelete}
      softDeleteFn={transportRailWaybillSoftDelete}
      queryKey={transportRailWaybillListQueryKey()}
      entityLabel={t('transport:rail.singular')}
    />
  )
}
