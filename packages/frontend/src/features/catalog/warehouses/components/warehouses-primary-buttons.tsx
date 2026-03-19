import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useWarehouses } from './warehouses-provider'

export function WarehousesPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = useWarehouses()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:warehouse.create')}
    </Button>
  )
}
