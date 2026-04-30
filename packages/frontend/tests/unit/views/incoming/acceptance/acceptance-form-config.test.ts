import type { AcceptanceCreate } from '~/views/incoming/acceptance/acceptance-form-config'
import { describe, expect, it } from 'vitest'
import {

  acceptanceCreateSchema,
  acceptanceUpdateSchema,
  emptyAcceptanceCreate,
} from '~/views/incoming/acceptance/acceptance-form-config'

const UUID_CONTRACTOR = '11111111-1111-4111-8111-111111111111'
const UUID_PRODUCT = '22222222-2222-4222-8222-222222222222'
const UUID_STORAGE = '33333333-3333-4333-8333-333333333333'
const UUID_TRUCK_WAYBILL = '44444444-4444-4444-8444-444444444444'
const UUID_RAIL_WAYBILL = '55555555-5555-4555-8555-555555555555'
const UUID_ITEM = '66666666-6666-4666-8666-666666666666'

describe('acceptanceCreate type and emptyAcceptanceCreate', () => {
  it('emptyAcceptanceCreate has truckWaybillId: null', () => {
    expect(emptyAcceptanceCreate.truckWaybillId).toBeNull()
  })

  it('emptyAcceptanceCreate has railWaybillId: null', () => {
    expect(emptyAcceptanceCreate.railWaybillId).toBeNull()
  })

  it('acceptanceCreate accepts truckWaybillId as string | null', () => {
    const value: AcceptanceCreate = {
      ...emptyAcceptanceCreate,
      truckWaybillId: 'some-uuid',
      arrivalType: 'TRUCK',
    }
    expect(value.truckWaybillId).toBe('some-uuid')
  })

  it('acceptanceCreate accepts railWaybillId as string | null', () => {
    const value: AcceptanceCreate = {
      ...emptyAcceptanceCreate,
      railWaybillId: 'some-uuid',
      arrivalType: 'RAIL',
    }
    expect(value.railWaybillId).toBe('some-uuid')
  })
})

describe('acceptanceCreateSchema discriminated union', () => {
  const baseValid = {
    documentNumber: 'ACC-001',
    dateAccepted: '2026-04-30T00:00:00Z',
    contractorId: UUID_CONTRACTOR,
    items: [{ productId: UUID_PRODUCT, storageId: UUID_STORAGE, acceptedAmount: '10' }],
    sourceEntity: null,
    truckWaybillId: null,
    railWaybillId: null,
  }

  it('accepts arrivalType=EXTERNAL with sourceEntity set and FKs null', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'EXTERNAL',
      sourceEntity: 'Some external supplier',
    })
    expect(result.success).toBe(true)
  })

  it('accepts arrivalType=EXTERNAL with sourceEntity null and FKs null', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'EXTERNAL',
    })
    expect(result.success).toBe(true)
  })

  it('accepts arrivalType=TRUCK with truckWaybillId set and others null', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'TRUCK',
      truckWaybillId: UUID_TRUCK_WAYBILL,
    })
    expect(result.success).toBe(true)
  })

  it('accepts arrivalType=RAIL with railWaybillId set and others null', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'RAIL',
      railWaybillId: UUID_RAIL_WAYBILL,
    })
    expect(result.success).toBe(true)
  })

  it('rejects arrivalType=TRUCK with truckWaybillId null', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'TRUCK',
    })
    expect(result.success).toBe(false)
    if (!result.success) {
      const paths = result.error.issues.map(i => i.path.join('.'))
      expect(paths).toContain('truckWaybillId')
    }
  })

  it('rejects arrivalType=TRUCK with both truck and rail FKs set', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'TRUCK',
      truckWaybillId: UUID_TRUCK_WAYBILL,
      railWaybillId: UUID_RAIL_WAYBILL,
    })
    expect(result.success).toBe(false)
    if (!result.success) {
      const paths = result.error.issues.map(i => i.path.join('.'))
      expect(paths).toContain('railWaybillId')
    }
  })

  it('rejects arrivalType=TRUCK with sourceEntity set', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'TRUCK',
      truckWaybillId: UUID_TRUCK_WAYBILL,
      sourceEntity: 'should not be set',
    })
    expect(result.success).toBe(false)
    if (!result.success) {
      const paths = result.error.issues.map(i => i.path.join('.'))
      expect(paths).toContain('sourceEntity')
    }
  })

  it('rejects arrivalType=RAIL with railWaybillId null', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'RAIL',
    })
    expect(result.success).toBe(false)
    if (!result.success) {
      const paths = result.error.issues.map(i => i.path.join('.'))
      expect(paths).toContain('railWaybillId')
    }
  })

  it('rejects arrivalType=EXTERNAL with truckWaybillId set', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'EXTERNAL',
      truckWaybillId: UUID_TRUCK_WAYBILL,
    })
    expect(result.success).toBe(false)
    if (!result.success) {
      const paths = result.error.issues.map(i => i.path.join('.'))
      expect(paths).toContain('truckWaybillId')
    }
  })

  it('rejects arrivalType=EXTERNAL with railWaybillId set', () => {
    const result = acceptanceCreateSchema.safeParse({
      ...baseValid,
      arrivalType: 'EXTERNAL',
      railWaybillId: UUID_RAIL_WAYBILL,
    })
    expect(result.success).toBe(false)
  })
})

describe('acceptanceUpdateSchema discriminated union (mirrors create)', () => {
  const baseValid = {
    documentNumber: 'ACC-001',
    dateAccepted: '2026-04-30T00:00:00Z',
    contractorId: UUID_CONTRACTOR,
    items: [{
      id: UUID_ITEM,
      productId: UUID_PRODUCT,
      storageId: UUID_STORAGE,
      acceptedAmount: '10',
    }],
    sourceEntity: null,
    truckWaybillId: null,
    railWaybillId: null,
  }

  it('accepts a valid TRUCK update with truckWaybillId set', () => {
    const result = acceptanceUpdateSchema.safeParse({
      ...baseValid,
      arrivalType: 'TRUCK',
      truckWaybillId: UUID_TRUCK_WAYBILL,
    })
    expect(result.success).toBe(true)
  })

  it('rejects a TRUCK update with truckWaybillId null', () => {
    const result = acceptanceUpdateSchema.safeParse({
      ...baseValid,
      arrivalType: 'TRUCK',
    })
    expect(result.success).toBe(false)
  })

  it('accepts an EXTERNAL update with sourceEntity set', () => {
    const result = acceptanceUpdateSchema.safeParse({
      ...baseValid,
      arrivalType: 'EXTERNAL',
      sourceEntity: 'External supplier',
    })
    expect(result.success).toBe(true)
  })
})
