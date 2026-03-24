import type { StorageResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type StoragesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: StoragesProvider, useEntity: useStorages }
  = createEntityProvider<StorageResponse, StoragesDialogType>('Storages')
