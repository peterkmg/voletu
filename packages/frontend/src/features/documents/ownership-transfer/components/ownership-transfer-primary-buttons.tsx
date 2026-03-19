import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useOwnershipTransfer } from './ownership-transfer-provider'

export function OwnershipTransferPrimaryButtons() {
  const { t } = useTranslation(['documents'])
  const { setOpen } = useOwnershipTransfer()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('documents:ownershipTransfer.create')}
    </Button>
  )
}
