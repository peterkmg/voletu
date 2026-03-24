import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { OwnershipTransferDialogs } from './components/ownership-transfer-dialogs'
import { OwnershipTransferPrimaryButtons } from './components/ownership-transfer-primary-buttons'
import { OwnershipTransferProvider } from './components/ownership-transfer-provider'
import { OwnershipTransferTable } from './components/ownership-transfer-table'

export function OwnershipTransfers() {
  const { t } = useTranslation(['documents'])
  const queryResult = useOwnershipTransferList()

  return (
    <EntityPage
      provider={OwnershipTransferProvider}
      title={t('documents:ownershipTransfer.title')}
      queryResult={queryResult}
      primaryButtons={OwnershipTransferPrimaryButtons}
      table={OwnershipTransferTable}
      dialogs={OwnershipTransferDialogs}
    />
  )
}
