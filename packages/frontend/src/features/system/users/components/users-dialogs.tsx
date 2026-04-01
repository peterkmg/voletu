import { systemUserDelete } from '~/generated/client'
import { systemUserListQueryKey } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { UserCreateDrawer } from './user-create-drawer'
import { useUsers } from './users-provider'

const UserDeleteDialog = createDeleteDialog({
  useEntity: useUsers,
  hardDeleteFn: systemUserDelete,
  queryKey: systemUserListQueryKey,
  entityLabel: 'system:users.singular',
  i18nNamespaces: ['common', 'system'],
})

export const UsersDialogs = createEntityDialogs({
  useEntity: useUsers,
  MutateDialog: UserCreateDrawer,
  DeleteDialog: UserDeleteDialog,
  supportsUpdate: false,
})
