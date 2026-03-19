import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useProductTypes } from './product-types-provider'

export function ProductTypesPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = useProductTypes()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:productType.create')}
    </Button>
  )
}
