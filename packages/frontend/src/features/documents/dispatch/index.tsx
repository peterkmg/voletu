import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useDispatchDocumentList } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { DispatchDialogs } from './components/dispatch-dialogs'
import { DispatchPrimaryButtons } from './components/dispatch-primary-buttons'
import { DispatchProvider } from './components/dispatch-provider'
import { DispatchTable } from './components/dispatch-table'

export function DispatchDocuments() {
  const { t } = useTranslation(['documents'])
  const queryResult = useDispatchDocumentList()

  return (
    <EntityPage
      provider={DispatchProvider}
      title={t('documents:dispatch.title')}
      queryResult={queryResult}
      primaryButtons={DispatchPrimaryButtons}
      table={DispatchTable}
      dialogs={DispatchDialogs}
    />
  )
}
