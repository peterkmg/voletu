import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { PhysicalTransferDialogs } from './components/physical-transfer-dialogs'
import { PhysicalTransferPrimaryButtons } from './components/physical-transfer-primary-buttons'
import { PhysicalTransferProvider } from './components/physical-transfer-provider'
import { PhysicalTransferTable } from './components/physical-transfer-table'

export function PhysicalTransfers() {
  const { t } = useTranslation(['documents'])

  const { data: listData, isLoading } = usePhysicalTransferList()
  const physicalTransfers = listData?.data ?? []

  return (
    <PhysicalTransferProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('documents:physicalTransfer.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('documents:physicalTransfer.title')}
            </h2>
          </div>
          <PhysicalTransferPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <PhysicalTransferTable data={physicalTransfers} />
            )}
      </Main>

      <PhysicalTransferDialogs />
    </PhysicalTransferProvider>
  )
}
