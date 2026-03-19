import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { WarehousesDialogs } from './components/warehouses-dialogs'
import { WarehousesPrimaryButtons } from './components/warehouses-primary-buttons'
import { WarehousesProvider } from './components/warehouses-provider'
import { WarehousesTable } from './components/warehouses-table'

export function Warehouses() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogWarehouseList()
  const warehouses = listData?.data ?? []

  return (
    <WarehousesProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('catalog:warehouse.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:warehouse.title')}
            </h2>
          </div>
          <WarehousesPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <WarehousesTable data={warehouses} />
            )}
      </Main>

      <WarehousesDialogs />
    </WarehousesProvider>
  )
}
