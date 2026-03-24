import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useReconciliationList } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { ReconciliationDialogs } from './components/reconciliation-dialogs'
import { ReconciliationPrimaryButtons } from './components/reconciliation-primary-buttons'
import { ReconciliationProvider } from './components/reconciliation-provider'
import { ReconciliationTable } from './components/reconciliation-table'

export function InventoryReconciliation() {
  const { t } = useTranslation(['documents'])
  const queryResult = useReconciliationList()

  return (
    <EntityPage
      provider={ReconciliationProvider}
      title={t('documents:reconciliation.title')}
      queryResult={queryResult}
      primaryButtons={ReconciliationPrimaryButtons}
      table={ReconciliationTable}
      dialogs={ReconciliationDialogs}
    />
  )
}
