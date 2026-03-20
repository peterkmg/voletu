import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { BasesDialogs } from './components/bases-dialogs'
import { BasesPrimaryButtons } from './components/bases-primary-buttons'
import { BasesProvider } from './components/bases-provider'
import { BasesTable } from './components/bases-table'

export function Bases() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogBaseList()
  const bases = listData?.data ?? []

  return (
    <BasesProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:base.title')}
            </h2>
          </div>
          <BasesPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <BasesTable data={bases} />
            )}
      </Main>

      <BasesDialogs />
    </BasesProvider>
  )
}
