import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'

interface PrimaryButtonsConfig<TDialogType extends string> {
  useEntity: () => { setOpen: (v: TDialogType | null) => void }
  createLabel: string
  i18nNamespaces: readonly [string, ...string[]]
}

export function createPrimaryButtons<TDialogType extends string>(
  config: PrimaryButtonsConfig<TDialogType>,
) {
  const { useEntity, createLabel, i18nNamespaces } = config

  function PrimaryButtons() {
    const { t } = useTranslation(i18nNamespaces as unknown as string[])
    const { setOpen } = useEntity()

    return (
      <Button onClick={() => setOpen('create' as TDialogType)}>
        <Plus className="mr-2 size-4" />
        {t(createLabel)}
      </Button>
    )
  }

  return PrimaryButtons
}
