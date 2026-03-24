import type { BlendingResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useBlending } from './blending-provider'

export const DataTableRowActions = createRowActions<BlendingResponse>({
  useEntity: useBlending,
  lifecycle: true,
})
