import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { ProductTypesDialogs } from './components/product-types-dialogs'
import { ProductTypesPrimaryButtons } from './components/product-types-primary-buttons'
import { ProductTypesProvider } from './components/product-types-provider'
import { ProductTypesTable } from './components/product-types-table'

export function ProductTypes() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogProductTypeList()
  const productTypes = listData?.data ?? []

  return (
    <ProductTypesProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('catalog:productType.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:productType.title')}
            </h2>
          </div>
          <ProductTypesPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <ProductTypesTable data={productTypes} />
            )}
      </Main>

      <ProductTypesDialogs />
    </ProductTypesProvider>
  )
}
