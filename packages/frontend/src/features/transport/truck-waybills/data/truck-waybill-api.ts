import {
  transportTruckWaybillCreate,
  transportTruckWaybillHardDelete,
  transportTruckWaybillSoftDelete,
  transportTruckWaybillUpdate,
} from '~/generated/client'
import { transportTruckWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateTruckWaybillRequest } from '~/generated/types/CreateTruckWaybillRequest'
export type { TruckWaybillResponse } from '~/generated/types/TruckWaybillResponse'
export type { UpdateTruckWaybillRequest } from '~/generated/types/UpdateTruckWaybillRequest'

export const createTruckWaybill = transportTruckWaybillCreate
export const updateTruckWaybill = transportTruckWaybillUpdate
export const softDeleteTruckWaybill = transportTruckWaybillSoftDelete
export const hardDeleteTruckWaybill = transportTruckWaybillHardDelete

export function invalidateTruckWaybills() {
  return queryClient.invalidateQueries({ queryKey: transportTruckWaybillListQueryKey() })
}
