import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { ProductsDialogs } from './components/products-dialogs'
import { ProductsPrimaryButtons } from './components/products-primary-buttons'
import { ProductsProvider } from './components/products-provider'
import { ProductsTable } from './components/products-table'

export function Products() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogProductList()

  return (
    <EntityPage
      provider={ProductsProvider}
      title={t('catalog:product.title')}
      queryResult={queryResult}
      primaryButtons={ProductsPrimaryButtons}
      table={ProductsTable}
      dialogs={ProductsDialogs}
    />
  )
}
