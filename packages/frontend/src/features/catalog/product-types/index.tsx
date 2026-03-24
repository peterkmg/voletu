import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { ProductTypesDialogs } from './components/product-types-dialogs'
import { ProductTypesPrimaryButtons } from './components/product-types-primary-buttons'
import { ProductTypesProvider } from './components/product-types-provider'
import { ProductTypesTable } from './components/product-types-table'

export function ProductTypes() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogProductTypeList()

  return (
    <EntityPage
      provider={ProductTypesProvider}
      title={t('catalog:productType.title')}
      queryResult={queryResult}
      primaryButtons={ProductTypesPrimaryButtons}
      table={ProductTypesTable}
      dialogs={ProductTypesDialogs}
    />
  )
}
