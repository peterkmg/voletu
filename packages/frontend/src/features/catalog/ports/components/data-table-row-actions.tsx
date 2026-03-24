import type { PortResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { usePorts } from './ports-provider'

export const DataTableRowActions = createRowActions<PortResponse>({
  useEntity: usePorts,
})
