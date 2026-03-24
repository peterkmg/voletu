import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useTransportTruckWaybillList } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { TruckWaybillsDialogs } from './components/truck-waybills-dialogs'
import { TruckWaybillsPrimaryButtons } from './components/truck-waybills-primary-buttons'
import { TruckWaybillsProvider } from './components/truck-waybills-provider'
import { TruckWaybillsTable } from './components/truck-waybills-table'

export function TruckWaybills() {
  const { t } = useTranslation(['transport'])
  const queryResult = useTransportTruckWaybillList()

  return (
    <EntityPage
      provider={TruckWaybillsProvider}
      title={t('transport:truck.title')}
      queryResult={queryResult}
      primaryButtons={TruckWaybillsPrimaryButtons}
      table={TruckWaybillsTable}
      dialogs={TruckWaybillsDialogs}
    />
  )
}
