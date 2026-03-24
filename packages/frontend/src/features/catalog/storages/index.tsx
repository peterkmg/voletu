import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { StoragesDialogs } from './components/storages-dialogs'
import { StoragesPrimaryButtons } from './components/storages-primary-buttons'
import { StoragesProvider } from './components/storages-provider'
import { StoragesTable } from './components/storages-table'

export function Storages() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogStorageList()

  return (
    <EntityPage
      provider={StoragesProvider}
      title={t('catalog:storage.title')}
      queryResult={queryResult}
      primaryButtons={StoragesPrimaryButtons}
      table={StoragesTable}
      dialogs={StoragesDialogs}
    />
  )
}
