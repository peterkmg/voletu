import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { useCompanies } from './companies-provider'

export function CompaniesPrimaryButtons() {
  const { t } = useTranslation(['catalog'])
  const { setOpen } = useCompanies()

  return (
    <Button onClick={() => setOpen('create')}>
      <Plus className="mr-2 size-4" />
      {t('catalog:company.create')}
    </Button>
  )
}
