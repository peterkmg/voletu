import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { OwnershipTransferDialogs } from './components/ownership-transfer-dialogs'
import { OwnershipTransferPrimaryButtons } from './components/ownership-transfer-primary-buttons'
import { OwnershipTransferProvider } from './components/ownership-transfer-provider'
import { OwnershipTransferTable } from './components/ownership-transfer-table'

export function OwnershipTransfers() {
  const { t } = useTranslation(['documents'])

  const { data: listData, isLoading } = useOwnershipTransferList()
  const ownershipTransfers = listData?.data ?? []

  return (
    <OwnershipTransferProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('documents:ownershipTransfer.title')}
            </h2>
          </div>
          <OwnershipTransferPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <OwnershipTransferTable data={ownershipTransfers} />
            )}
      </Main>

      <OwnershipTransferDialogs />
    </OwnershipTransferProvider>
  )
}
