import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useStorages } from './storages-provider'

export const StoragesPrimaryButtons = createPrimaryButtons({
  useEntity: useStorages,
  createLabel: 'catalog:storage.create',
  i18nNamespaces: ['catalog'],
})
