import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useAcceptance } from './acceptance-provider'

export function AcceptancePrimaryButtons() {
  const { t } = useTranslation(['documents'])
  const { setOpen } = useAcceptance()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('documents:acceptance.create')}
    </Button>
  )
}
