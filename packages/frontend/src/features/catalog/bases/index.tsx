import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { BasesDialogs } from './components/bases-dialogs'
import { BasesPrimaryButtons } from './components/bases-primary-buttons'
import { BasesProvider } from './components/bases-provider'
import { BasesTable } from './components/bases-table'

export function Bases() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogBaseList()

  return (
    <EntityPage
      provider={BasesProvider}
      title={t('catalog:base.title')}
      queryResult={queryResult}
      primaryButtons={BasesPrimaryButtons}
      table={BasesTable}
      dialogs={BasesDialogs}
    />
  )
}
