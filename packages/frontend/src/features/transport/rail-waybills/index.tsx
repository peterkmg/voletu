import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useTransportRailWaybillList } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { RailWaybillsDialogs } from './components/rail-waybills-dialogs'
import { RailWaybillsPrimaryButtons } from './components/rail-waybills-primary-buttons'
import { RailWaybillsProvider } from './components/rail-waybills-provider'
import { RailWaybillsTable } from './components/rail-waybills-table'

export function RailWaybills() {
  const { t } = useTranslation(['transport'])

  const { data: listData, isLoading } = useTransportRailWaybillList()
  const railWaybills = listData?.data ?? []

  return (
    <RailWaybillsProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('transport:rail.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('transport:rail.title')}
            </h2>
          </div>
          <RailWaybillsPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <RailWaybillsTable data={railWaybills} />
            )}
      </Main>

      <RailWaybillsDialogs />
    </RailWaybillsProvider>
  )
}
