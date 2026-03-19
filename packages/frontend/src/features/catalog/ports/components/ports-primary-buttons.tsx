import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { usePorts } from './ports-provider'

export function PortsPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = usePorts()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:port.create')}
    </Button>
  )
}
