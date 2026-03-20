import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { CompaniesDialogs } from './components/companies-dialogs'
import { CompaniesPrimaryButtons } from './components/companies-primary-buttons'
import { CompaniesProvider } from './components/companies-provider'
import { CompaniesTable } from './components/companies-table'

export function Companies() {
  const { t } = useTranslation(['catalog'])

  const { data: listData, isLoading } = useCatalogCompanyList()
  const companies = listData?.data ?? []

  return (
    <CompaniesProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('catalog:company.title')}
            </h2>
          </div>
          <CompaniesPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <CompaniesTable data={companies} />
            )}
      </Main>

      <CompaniesDialogs />
    </CompaniesProvider>
  )
}
