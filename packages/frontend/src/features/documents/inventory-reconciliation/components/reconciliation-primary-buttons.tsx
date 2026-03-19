import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useReconciliation } from './reconciliation-provider'

export function ReconciliationPrimaryButtons() {
  const { t } = useTranslation(['documents'])
  const { setOpen } = useReconciliation()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('documents:reconciliation.create')}
    </Button>
  )
}
