import type { BaseResponse } from '~/generated/types/BaseResponse'
import { createRowActions } from '~/lib/create-row-actions'
import { useBases } from './bases-provider'

export const DataTableRowActions = createRowActions<BaseResponse>({
  useEntity: useBases,
})
