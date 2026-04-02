import { useTranslation } from 'react-i18next'

export function CargoFlowPage() {
  const { t } = useTranslation(['common'])

  return (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className="text-lg font-semibold">{t('common:nav.cargoFlow')}</h1>
      <p className="text-muted-foreground">Unified view across all document types — coming soon.</p>
    </div>
  )
}
