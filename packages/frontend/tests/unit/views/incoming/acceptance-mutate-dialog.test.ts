import type { AcceptanceItemResponse } from '~/generated/types'
import { describe, expect, it } from 'vitest'
import { acceptanceItemSchema } from '~/views/incoming/acceptance/acceptance-form-config'
import { toAcceptanceItemFormValue } from '~/views/incoming/acceptance/acceptance-item-mapping'

describe('acceptance mutate dialog item mapping', () => {
  const itemId = '018f4cf7-68d4-7d2a-bc6c-8c9a58f25b01'
  const documentId = '018f4cf7-68d4-7d2a-bc6c-8c9a58f25b02'
  const productId = '018f4cf7-68d4-7d2a-bc6c-8c9a58f25b03'
  const storageId = '018f4cf7-68d4-7d2a-bc6c-8c9a58f25b04'
  const originDbId = '018f4cf7-68d4-7d2a-bc6c-8c9a58f25b05'
  const userId = '018f4cf7-68d4-7d2a-bc6c-8c9a58f25b06'

  it('preserves item id for edit-mode update payloads', () => {
    const item: AcceptanceItemResponse = {
      id: itemId,
      acceptanceDocId: documentId,
      productId,
      storageId,
      acceptedAmount: '12.5',
      originDbId,
      createdAt: '2026-04-29T00:00:00Z',
      createdBy: userId,
      updatedAt: '2026-04-29T00:00:00Z',
      updatedBy: userId,
    }

    expect(toAcceptanceItemFormValue(item)).toEqual({
      id: itemId,
      productId,
      storageId,
      acceptedAmount: '12.5',
    })
  })

  it('keeps item id when validating edited row values', () => {
    expect(
      acceptanceItemSchema.parse({
        id: itemId,
        productId,
        storageId,
        acceptedAmount: '12.5',
      }),
    ).toEqual({
      id: itemId,
      productId,
      storageId,
      acceptedAmount: '12.5',
    })
  })
})
