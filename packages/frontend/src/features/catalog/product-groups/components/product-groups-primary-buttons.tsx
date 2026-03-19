import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useProductGroups } from './product-groups-provider'

export function ProductGroupsPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = useProductGroups()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:productGroup.create')}
    </Button>
  )
}
