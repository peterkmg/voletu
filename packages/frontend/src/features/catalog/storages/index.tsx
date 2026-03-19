import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { StoragesDialogs } from './components/storages-dialogs'
import { StoragesPrimaryButtons } from './components/storages-primary-buttons'
import { StoragesProvider } from './components/storages-provider'
import { StoragesTable } from './components/storages-table'

export function Storages() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogStorageList()
  const storages = listData?.data ?? []

  return (
    <StoragesProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('catalog:storage.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:storage.title')}
            </h2>
          </div>
          <StoragesPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <StoragesTable data={storages} />
            )}
      </Main>

      <StoragesDialogs />
    </StoragesProvider>
  )
}
