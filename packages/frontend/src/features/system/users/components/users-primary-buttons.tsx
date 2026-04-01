import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useUsers } from './users-provider'

export const UsersPrimaryButtons = createPrimaryButtons({
  useEntity: useUsers,
  createLabel: 'system:users.create',
  i18nNamespaces: ['system'],
})
