import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useLedgerBalanceList } from '~/generated/hooks/LedgerHooks/useLedgerBalanceList'
import { LedgerTable } from './components/ledger-table'

export function Ledger() {
  const { t } = useTranslation(['system', 'common'])

  const { data: listData, isLoading } = useLedgerBalanceList()
  const balances = listData?.data ?? []

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
                <div className="text-muted-foreground">{t('common:loading')}</div>
              </div>
            )
          : (
              <LedgerTable data={balances} />
            )}
      </Main>
    </>
  )
}
