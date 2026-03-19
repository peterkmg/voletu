import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useReconciliationList } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { ReconciliationDialogs } from './components/reconciliation-dialogs'
import { ReconciliationPrimaryButtons } from './components/reconciliation-primary-buttons'
import { ReconciliationProvider } from './components/reconciliation-provider'
import { ReconciliationTable } from './components/reconciliation-table'

export function InventoryReconciliation() {
  const { t } = useTranslation(['documents'])

  const { data: listData, isLoading } = useReconciliationList()
  const documents = listData?.data ?? []

  return (
    <ReconciliationProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('documents:reconciliation.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('documents:reconciliation.title')}
            </h2>
          </div>
          <ReconciliationPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <ReconciliationTable data={documents} />
            )}
      </Main>

      <ReconciliationDialogs />
    </ReconciliationProvider>
  )
}
