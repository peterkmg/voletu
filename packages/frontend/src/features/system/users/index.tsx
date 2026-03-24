import { useTranslation } from 'react-i18next'
import { EntityPage } from '~/components/entity-page'
import { useSystemUserList } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { UsersDialogs } from './components/users-dialogs'
import { UsersPrimaryButtons } from './components/users-primary-buttons'
import { UsersProvider } from './components/users-provider'
import { UsersTable } from './components/users-table'

export function Users() {
  const { t } = useTranslation(['system'])
  const queryResult = useSystemUserList()

  return (
    <EntityPage
      provider={UsersProvider}
      title={t('system:users.title')}
      queryResult={queryResult}
      primaryButtons={UsersPrimaryButtons}
      table={UsersTable}
      dialogs={UsersDialogs}
    />
  )
}
