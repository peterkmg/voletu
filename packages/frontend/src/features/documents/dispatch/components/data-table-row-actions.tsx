import type { DispatchResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useDispatch } from './dispatch-provider'

export const DataTableRowActions = createRowActions<DispatchResponse>({
  useEntity: useDispatch,
  lifecycle: true,
})
