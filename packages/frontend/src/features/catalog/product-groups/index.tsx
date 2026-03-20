import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { ProductGroupsDialogs } from './components/product-groups-dialogs'
import { ProductGroupsPrimaryButtons } from './components/product-groups-primary-buttons'
import { ProductGroupsProvider } from './components/product-groups-provider'
import { ProductGroupsTable } from './components/product-groups-table'

export function ProductGroups() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogProductGroupList()
  const productGroups = listData?.data ?? []

  return (
    <ProductGroupsProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:productGroup.title')}
            </h2>
          </div>
          <ProductGroupsPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <ProductGroupsTable data={productGroups} />
            )}
      </Main>

      <ProductGroupsDialogs />
    </ProductGroupsProvider>
  )
}
