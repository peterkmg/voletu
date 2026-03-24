import type { StorageResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useStorages } from './storages-provider'

export const DataTableRowActions = createRowActions<StorageResponse>({
  useEntity: useStorages,
})
