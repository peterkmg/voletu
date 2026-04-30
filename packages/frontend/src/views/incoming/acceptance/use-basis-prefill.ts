import type { AcceptanceCreate, AcceptanceItem } from './acceptance-form-config'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { emptyAcceptanceCreate } from './acceptance-form-config'

export interface BasisPrefillRef {
  kind: 'truck' | 'rail'
  basisId: string
}

export interface BasisPrefillResult {

  isLoading: boolean

  prefilled: AcceptanceCreate | null

  lockedHintNumber: string | null
}

function emptyItemFor(productId: string): AcceptanceItem {
  return {
    productId,
    storageId: '',
    acceptedAmount: '',
  }
}

export function useBasisPrefill(
  ref: BasisPrefillRef | undefined,
  enabled: boolean,
): BasisPrefillResult {
  const wantTruck = enabled && ref?.kind === 'truck'
  const wantRail = enabled && ref?.kind === 'rail'

  const truckQ = useTransportTruckWaybillCompositeGet(
    wantTruck ? ref!.basisId : '',
    undefined,
    { query: { enabled: wantTruck } },
  )
  const railQ = useTransportRailWaybillCompositeGet(
    wantRail ? ref!.basisId : '',
    undefined,
    { query: { enabled: wantRail } },
  )

  if (!enabled || !ref) {
    return { isLoading: false, prefilled: null, lockedHintNumber: null }
  }

  if (wantTruck) {
    if (truckQ.isLoading || !truckQ.data?.data) {
      return { isLoading: true, prefilled: null, lockedHintNumber: null }
    }
    const composite = truckQ.data.data
    const wb = composite.waybill
    const items = (composite.items ?? []).map(it => emptyItemFor(it.productId))
    const prefilled: AcceptanceCreate = {
      ...emptyAcceptanceCreate,
      arrivalType: 'TRUCK',
      truckWaybillId: wb.id,
      contractorId: wb.senderId,
      items,
    }
    return {
      isLoading: false,
      prefilled,
      lockedHintNumber: wb.documentNumber ?? null,
    }
  }

  if (wantRail) {
    if (railQ.isLoading || !railQ.data?.data) {
      return { isLoading: true, prefilled: null, lockedHintNumber: null }
    }
    const composite = railQ.data.data
    const wb = composite.waybill
    const items = (composite.wagonManifests ?? []).map(m => emptyItemFor(m.productId))
    const prefilled: AcceptanceCreate = {
      ...emptyAcceptanceCreate,
      arrivalType: 'RAIL',
      railWaybillId: wb.id,
      contractorId: wb.senderId,
      items,
    }
    return {
      isLoading: false,
      prefilled,
      lockedHintNumber: wb.documentNumber ?? null,
    }
  }

  return { isLoading: false, prefilled: null, lockedHintNumber: null }
}
