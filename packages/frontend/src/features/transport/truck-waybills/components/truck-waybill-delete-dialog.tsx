import type { TruckWaybillResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { transportTruckWaybillHardDelete, transportTruckWaybillSoftDelete } from '~/generated/client'
import { transportTruckWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'

interface TruckWaybillDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: TruckWaybillResponse | null
  variant: 'soft' | 'hard'
}

export function TruckWaybillDeleteDialog({ open, onOpenChange, currentRow, variant }: TruckWaybillDeleteDialogProps) {
  const { t } = useTranslation(['common', 'transport'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={transportTruckWaybillHardDelete}
      softDeleteFn={transportTruckWaybillSoftDelete}
      queryKey={transportTruckWaybillListQueryKey()}
      entityLabel={t('transport:truck.singular')}
    />
  )
}
