import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useRailWaybills } from './rail-waybills-provider'

export function RailWaybillsPrimaryButtons() {
  const { t } = useTranslation(['transport'])
  const { setOpen } = useRailWaybills()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('transport:rail.create')}
    </Button>
  )
}
