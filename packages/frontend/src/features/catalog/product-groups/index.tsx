import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { ProductGroupsDialogs } from './components/product-groups-dialogs'
import { ProductGroupsPrimaryButtons } from './components/product-groups-primary-buttons'
import { ProductGroupsProvider } from './components/product-groups-provider'
import { ProductGroupsTable } from './components/product-groups-table'

export function ProductGroups() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogProductGroupList()

  return (
    <EntityPage
      provider={ProductGroupsProvider}
      title={t('catalog:productGroup.title')}
      queryResult={queryResult}
      primaryButtons={ProductGroupsPrimaryButtons}
      table={ProductGroupsTable}
      dialogs={ProductGroupsDialogs}
    />
  )
}
