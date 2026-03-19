import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { ProductsDialogs } from './components/products-dialogs'
import { ProductsPrimaryButtons } from './components/products-primary-buttons'
import { ProductsProvider } from './components/products-provider'
import { ProductsTable } from './components/products-table'

export function Products() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogProductList()
  const products = listData?.data ?? []

  return (
    <ProductsProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('catalog:product.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:product.title')}
            </h2>
          </div>
          <ProductsPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <ProductsTable data={products} />
            )}
      </Main>

      <ProductsDialogs />
    </ProductsProvider>
  )
}
