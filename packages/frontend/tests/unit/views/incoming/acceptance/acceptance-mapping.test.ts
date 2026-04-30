import type { AcceptanceCompositeResponse } from '~/generated/types/AcceptanceCompositeResponse'
import { describe, expect, it } from 'vitest'
import { toAcceptanceFormValue } from '~/views/incoming/acceptance/acceptance-item-mapping'

const UUID_DOC = 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa'
const UUID_CONTRACTOR = '11111111-1111-4111-8111-111111111111'
const UUID_TRUCK_WAYBILL = '44444444-4444-4444-8444-444444444444'
const UUID_RAIL_WAYBILL = '55555555-5555-4555-8555-555555555555'
const UUID_ITEM = '66666666-6666-4666-8666-666666666666'
const UUID_PRODUCT = '22222222-2222-4222-8222-222222222222'
const UUID_STORAGE = '33333333-3333-4333-8333-333333333333'

function baseLoaded(): AcceptanceCompositeResponse {
  return {
    id: UUID_DOC,
    originDbId: UUID_DOC,
    documentNumber: 'ACC-001',
    dateAccepted: '2026-04-30T00:00:00Z',
    arrivalType: 'EXTERNAL',
    contractorId: UUID_CONTRACTOR,
    sourceEntity: null,
    status: 'DRAFT',
    createdAt: '2026-04-30T00:00:00Z',
    createdBy: UUID_CONTRACTOR,
    updatedAt: '2026-04-30T00:00:00Z',
    updatedBy: UUID_CONTRACTOR,
    items: [
      {
        id: UUID_ITEM,
        acceptanceDocId: UUID_DOC,
        originDbId: UUID_ITEM,
        productId: UUID_PRODUCT,
        storageId: UUID_STORAGE,
        acceptedAmount: '10',
        createdAt: '2026-04-30T00:00:00Z',
        createdBy: UUID_CONTRACTOR,
        updatedAt: '2026-04-30T00:00:00Z',
        updatedBy: UUID_CONTRACTOR,
      },
    ],
  }
}

describe('toAcceptanceFormValue', () => {
  it('round-trips truckWaybillId for a TRUCK acceptance', () => {
    const loaded = baseLoaded()
    loaded.arrivalType = 'TRUCK'
    loaded.truckWaybillId = UUID_TRUCK_WAYBILL

    const form = toAcceptanceFormValue(loaded)
    expect(form.truckWaybillId).toBe(UUID_TRUCK_WAYBILL)
    expect(form.railWaybillId).toBeNull()
    expect(form.arrivalType).toBe('TRUCK')
  })

  it('round-trips railWaybillId for a RAIL acceptance', () => {
    const loaded = baseLoaded()
    loaded.arrivalType = 'RAIL'
    loaded.railWaybillId = UUID_RAIL_WAYBILL

    const form = toAcceptanceFormValue(loaded)
    expect(form.railWaybillId).toBe(UUID_RAIL_WAYBILL)
    expect(form.truckWaybillId).toBeNull()
    expect(form.arrivalType).toBe('RAIL')
  })

  it('normalizes missing truckWaybillId / railWaybillId to null (EXTERNAL)', () => {
    const form = toAcceptanceFormValue(baseLoaded())
    expect(form.truckWaybillId).toBeNull()
    expect(form.railWaybillId).toBeNull()
    expect(form.sourceEntity).toBeNull()
  })

  it('preserves item ids so the backend can diff in place', () => {
    const form = toAcceptanceFormValue(baseLoaded())
    expect(form.items).toHaveLength(1)
    expect((form.items[0] as { id?: string }).id).toBe(UUID_ITEM)
  })

  it('round-trips sourceEntity for an EXTERNAL acceptance', () => {
    const loaded = baseLoaded()
    loaded.sourceEntity = 'External supplier'
    const form = toAcceptanceFormValue(loaded)
    expect(form.sourceEntity).toBe('External supplier')
  })
})
