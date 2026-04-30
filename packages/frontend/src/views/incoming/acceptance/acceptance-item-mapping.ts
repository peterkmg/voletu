import type { AcceptanceCreate, AcceptanceItem } from './acceptance-form-config'
import type { AcceptanceItemResponse } from '~/generated/types'
import type { AcceptanceCompositeResponse } from '~/generated/types/AcceptanceCompositeResponse'

/** Drop server-only fields while preserving the row id required by composite updates. */
export function toAcceptanceItemFormValue(item: AcceptanceItemResponse): AcceptanceItem {
  return {
    id: item.id,
    productId: item.productId,
    storageId: item.storageId,
    acceptedAmount: item.acceptedAmount,
  }
}

/**
 * Map a loaded acceptance composite to the form-shape used by edit-mode
 * `defaultValues`. Optional FK fields (`sourceEntity`, `truckWaybillId`,
 * `railWaybillId`) are normalized to `null` so the form's discriminated-union
 * refine sees the same shape as create-mode.
 */
export function toAcceptanceFormValue(loaded: AcceptanceCompositeResponse): AcceptanceCreate {
  return {
    documentNumber: loaded.documentNumber,
    dateAccepted: loaded.dateAccepted,
    arrivalType: loaded.arrivalType,
    contractorId: loaded.contractorId,
    sourceEntity: loaded.sourceEntity ?? null,
    truckWaybillId: loaded.truckWaybillId ?? null,
    railWaybillId: loaded.railWaybillId ?? null,
    items: loaded.items.map(toAcceptanceItemFormValue),
  }
}
