import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogPortList } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { PortsDialogs } from './components/ports-dialogs'
import { PortsPrimaryButtons } from './components/ports-primary-buttons'
import { PortsProvider } from './components/ports-provider'
import { PortsTable } from './components/ports-table'

export function Ports() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogPortList()

  return (
    <EntityPage
      provider={PortsProvider}
      title={t('catalog:port.title')}
      queryResult={queryResult}
      primaryButtons={PortsPrimaryButtons}
      table={PortsTable}
      dialogs={PortsDialogs}
    />
  )
}
