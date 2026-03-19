import {
  systemUserCreate,
  systemUserDelete,
} from '~/generated/client'
import { systemUserListQueryKey } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateUserRequest } from '~/generated/types/CreateUserRequest'
export type { UserResponse } from '~/generated/types/UserResponse'

export const createUser = systemUserCreate
export const softDeleteUser = systemUserDelete

export function invalidateUsers() {
  return queryClient.invalidateQueries({ queryKey: systemUserListQueryKey() })
}
