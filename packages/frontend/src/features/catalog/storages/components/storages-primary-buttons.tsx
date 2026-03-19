import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useStorages } from './storages-provider'

export function StoragesPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = useStorages()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:storage.create')}
    </Button>
  )
}
