import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useProducts } from './products-provider'

export function ProductsPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = useProducts()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:product.create')}
    </Button>
  )
}
