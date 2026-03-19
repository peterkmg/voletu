import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useBlending } from './blending-provider'

export function BlendingPrimaryButtons() {
  const { t } = useTranslation(['documents'])
  const { setOpen } = useBlending()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('documents:blending.create')}
    </Button>
  )
}
