import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useTransportRailWaybillList } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { RailWaybillsDialogs } from './components/rail-waybills-dialogs'
import { RailWaybillsPrimaryButtons } from './components/rail-waybills-primary-buttons'
import { RailWaybillsProvider } from './components/rail-waybills-provider'
import { RailWaybillsTable } from './components/rail-waybills-table'

export function RailWaybills() {
  const { t } = useTranslation(['transport'])
  const queryResult = useTransportRailWaybillList()

  return (
    <EntityPage
      provider={RailWaybillsProvider}
      title={t('transport:rail.title')}
      queryResult={queryResult}
      primaryButtons={RailWaybillsPrimaryButtons}
      table={RailWaybillsTable}
      dialogs={RailWaybillsDialogs}
    />
  )
}
