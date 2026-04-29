import type { AcceptanceItem } from './acceptance-form-config'
import type { AcceptanceItemResponse } from '~/generated/types'

/** Drop server-only fields while preserving the row id required by composite updates. */
export function toAcceptanceItemFormValue(item: AcceptanceItemResponse): AcceptanceItem {
  return {
    id: item.id,
    productId: item.productId,
    storageId: item.storageId,
    acceptedAmount: item.acceptedAmount,
  }
}
