import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useTransportTruckWaybillList } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { TruckWaybillsDialogs } from './components/truck-waybills-dialogs'
import { TruckWaybillsPrimaryButtons } from './components/truck-waybills-primary-buttons'
import { TruckWaybillsProvider } from './components/truck-waybills-provider'
import { TruckWaybillsTable } from './components/truck-waybills-table'

export function TruckWaybills() {
  const { t } = useTranslation(['transport'])

  const { data: listData, isLoading } = useTransportTruckWaybillList()
  const truckWaybills = listData?.data ?? []

  return (
    <TruckWaybillsProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('transport:truck.title')}
            </h2>
          </div>
          <TruckWaybillsPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <TruckWaybillsTable data={truckWaybills} />
            )}
      </Main>

      <TruckWaybillsDialogs />
    </TruckWaybillsProvider>
  )
}
