import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useTruckWaybills } from './truck-waybills-provider'

export function TruckWaybillsPrimaryButtons() {
  const { t } = useTranslation(['transport'])
  const { setOpen } = useTruckWaybills()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('transport:truck.create')}
    </Button>
  )
}
