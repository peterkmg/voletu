import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'

interface PrimaryButtonsConfig {
  useEntity: () => { openCreate: () => void }
}

export function createPrimaryButtons(config: PrimaryButtonsConfig) {
  function PrimaryButtons() {
    const { t } = useTranslation(['common'])
    const { openCreate } = config.useEntity()

    return (
      <Button size="sm" onClick={openCreate}>
        <Plus className="mr-2 size-4" />
        {t('common:actions.create')}
      </Button>
    )
  }

  return PrimaryButtons
}
