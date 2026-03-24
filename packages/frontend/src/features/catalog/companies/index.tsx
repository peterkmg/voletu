import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { CompaniesDialogs } from './components/companies-dialogs'
import { CompaniesPrimaryButtons } from './components/companies-primary-buttons'
import { CompaniesProvider } from './components/companies-provider'
import { CompaniesTable } from './components/companies-table'

export function Companies() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogCompanyList()

  return (
    <EntityPage
      provider={CompaniesProvider}
      title={t('catalog:company.title')}
      queryResult={queryResult}
      primaryButtons={CompaniesPrimaryButtons}
      table={CompaniesTable}
      dialogs={CompaniesDialogs}
    />
  )
}
