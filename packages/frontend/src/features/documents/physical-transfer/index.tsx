import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { PhysicalTransferDialogs } from './components/physical-transfer-dialogs'
import { PhysicalTransferPrimaryButtons } from './components/physical-transfer-primary-buttons'
import { PhysicalTransferProvider } from './components/physical-transfer-provider'
import { PhysicalTransferTable } from './components/physical-transfer-table'

export function PhysicalTransfers() {
  const { t } = useTranslation(['documents'])
  const queryResult = usePhysicalTransferList()

  return (
    <EntityPage
      provider={PhysicalTransferProvider}
      title={t('documents:physicalTransfer.title')}
      queryResult={queryResult}
      primaryButtons={PhysicalTransferPrimaryButtons}
      table={PhysicalTransferTable}
      dialogs={PhysicalTransferDialogs}
    />
  )
}
