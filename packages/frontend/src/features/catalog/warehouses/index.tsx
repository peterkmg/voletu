import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { WarehousesDialogs } from './components/warehouses-dialogs'
import { WarehousesPrimaryButtons } from './components/warehouses-primary-buttons'
import { WarehousesProvider } from './components/warehouses-provider'
import { WarehousesTable } from './components/warehouses-table'

export function Warehouses() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogWarehouseList()

  return (
    <EntityPage
      provider={WarehousesProvider}
      title={t('catalog:warehouse.title')}
      queryResult={queryResult}
      primaryButtons={WarehousesPrimaryButtons}
      table={WarehousesTable}
      dialogs={WarehousesDialogs}
    />
  )
}
