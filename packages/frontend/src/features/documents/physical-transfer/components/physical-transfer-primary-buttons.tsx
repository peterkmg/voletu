import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { usePhysicalTransfer } from './physical-transfer-provider'

export function PhysicalTransferPrimaryButtons() {
  const { t } = useTranslation(['documents'])
  const { setOpen } = usePhysicalTransfer()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('documents:physicalTransfer.create')}
    </Button>
  )
}
