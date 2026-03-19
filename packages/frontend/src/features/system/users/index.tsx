import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useSystemUserList } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { UsersDialogs } from './components/users-dialogs'
import { UsersPrimaryButtons } from './components/users-primary-buttons'
import { UsersProvider } from './components/users-provider'
import { UsersTable } from './components/users-table'

export function Users() {
  const { t } = useTranslation(['system'])

  const { data: listData, isLoading } = useSystemUserList()
  const users = listData?.data ?? []

  return (
    <UsersProvider>
      <Header fixed>
        <h1 className="text-lg font-semibold">{t('system:users.title')}</h1>
      </Header>

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('system:users.title')}
            </h2>
          </div>
          <UsersPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <UsersTable data={users} />
            )}
      </Main>

      <UsersDialogs />
    </UsersProvider>
  )
}
