import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogPortList } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { PortsDialogs } from './components/ports-dialogs'
import { PortsPrimaryButtons } from './components/ports-primary-buttons'
import { PortsProvider } from './components/ports-provider'
import { PortsTable } from './components/ports-table'

export function Ports() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogPortList()
  const ports = listData?.data ?? []

  return (
    <PortsProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('catalog:port.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:port.title')}
            </h2>
          </div>
          <PortsPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <PortsTable data={ports} />
            )}
      </Main>

      <PortsDialogs />
    </PortsProvider>
  )
}
