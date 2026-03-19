import {
  transportRailWaybillCreate,
  transportRailWaybillHardDelete,
  transportRailWaybillSoftDelete,
  transportRailWaybillUpdate,
} from '~/generated/client'
import { transportRailWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateRailWaybillRequest } from '~/generated/types/CreateRailWaybillRequest'
export type { RailWaybillResponse } from '~/generated/types/RailWaybillResponse'
export type { UpdateRailWaybillRequest } from '~/generated/types/UpdateRailWaybillRequest'

export const createRailWaybill = transportRailWaybillCreate
export const updateRailWaybill = transportRailWaybillUpdate
export const softDeleteRailWaybill = transportRailWaybillSoftDelete
export const hardDeleteRailWaybill = transportRailWaybillHardDelete

export function invalidateRailWaybills() {
  return queryClient.invalidateQueries({ queryKey: transportRailWaybillListQueryKey() })
}
