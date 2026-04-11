import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useLedgerEntryList } from '~/generated/hooks/LedgerHooks/useLedgerEntryList'
import { LedgerTable } from './components/ledger-table'

export function Ledger() {
  const { t } = useTranslation(['system'])

  const { data: listData, isLoading } = useLedgerEntryList()
  const entries = listData?.data ?? []

  return (
    <>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('system:ledger.title')}
            </h2>
          </div>
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <LedgerTable data={entries} />
            )}
      </Main>
    </>
  )
}
