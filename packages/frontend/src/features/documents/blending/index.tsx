import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useBlendingDocumentList } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { BlendingDialogs } from './components/blending-dialogs'
import { BlendingPrimaryButtons } from './components/blending-primary-buttons'
import { BlendingProvider } from './components/blending-provider'
import { BlendingTable } from './components/blending-table'

export function BlendingDocuments() {
  const { t } = useTranslation(['documents'])
  const queryResult = useBlendingDocumentList()

  return (
    <EntityPage
      provider={BlendingProvider}
      title={t('documents:blending.title')}
      queryResult={queryResult}
      primaryButtons={BlendingPrimaryButtons}
      table={BlendingTable}
      dialogs={BlendingDialogs}
    />
  )
}
