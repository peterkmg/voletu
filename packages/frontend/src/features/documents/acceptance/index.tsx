import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useAcceptanceDocumentList } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { AcceptanceDialogs } from './components/acceptance-dialogs'
import { AcceptancePrimaryButtons } from './components/acceptance-primary-buttons'
import { AcceptanceProvider } from './components/acceptance-provider'
import { AcceptanceTable } from './components/acceptance-table'

export function AcceptanceDocuments() {
  const { t } = useTranslation(['documents'])
  const queryResult = useAcceptanceDocumentList()

  return (
    <EntityPage
      provider={AcceptanceProvider}
      title={t('documents:acceptance.title')}
      queryResult={queryResult}
      primaryButtons={AcceptancePrimaryButtons}
      table={AcceptanceTable}
      dialogs={AcceptanceDialogs}
    />
  )
}
