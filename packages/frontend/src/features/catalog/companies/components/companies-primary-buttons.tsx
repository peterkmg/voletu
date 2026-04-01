import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useCompanies } from './companies-provider'

export const CompaniesPrimaryButtons = createPrimaryButtons({
  useEntity: useCompanies,
  createLabel: 'catalog:company.create',
  i18nNamespaces: ['catalog'],
})
