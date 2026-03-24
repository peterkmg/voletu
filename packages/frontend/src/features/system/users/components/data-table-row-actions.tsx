import type { UserResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useUsers } from './users-provider'

export const DataTableRowActions = createRowActions<UserResponse>({
  useEntity: useUsers,
  deleteOnly: true,
})
