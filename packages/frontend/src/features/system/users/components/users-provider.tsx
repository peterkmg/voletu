import type { UserResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type UsersDialogType = 'create' | 'delete'

export const { Provider: UsersProvider, useEntity: useUsers }
  = createEntityProvider<UserResponse, UsersDialogType>('Users')
