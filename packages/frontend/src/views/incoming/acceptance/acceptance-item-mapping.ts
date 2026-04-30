import type { AcceptanceCreate, AcceptanceItem } from './acceptance-form-config'
import type { AcceptanceItemResponse } from '~/generated/types'
import type { AcceptanceCompositeResponse } from '~/generated/types/AcceptanceCompositeResponse'

export function toAcceptanceItemFormValue(item: AcceptanceItemResponse): AcceptanceItem {
  return {
    id: item.id,
    productId: item.productId,
    storageId: item.storageId,
    acceptedAmount: item.acceptedAmount,
  }
}

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
