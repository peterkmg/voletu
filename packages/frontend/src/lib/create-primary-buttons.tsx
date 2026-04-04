import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'

interface PrimaryButtonsConfig<TDialogType extends string> {
  useEntity: () => { setOpen: (v: TDialogType | null) => void }
}

export function createPrimaryButtons<TDialogType extends string>(
  config: PrimaryButtonsConfig<TDialogType>,
) {
  function PrimaryButtons() {
    const { t } = useTranslation(['common'])
    const { setOpen } = config.useEntity()

    return (
      <Button size="sm" onClick={() => setOpen('create' as TDialogType)}>
        <Plus className="mr-2 size-4" />
        {t('common:actions.create')}
      </Button>
    )
  }

  return PrimaryButtons
}
