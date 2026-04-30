import type { AxisNode, BuilderInput } from '~/views/dashboard/types'

import { describe, expect, it } from 'vitest'
import { buildMatrixVM } from '~/views/dashboard/build-matrix-vm'

const CONTRACTOR_A = '00000000-0000-0000-0000-000000000001'
const CONTRACTOR_B = '00000000-0000-0000-0000-000000000002'
const PT_FUEL = 'pt-fuel'
const PT_OIL = 'pt-oil'
const PG_GASOLINE = 'pg-gasoline'
const PG_DIESEL = 'pg-diesel'
const PG_LUBE = 'pg-lube'
const P_95 = 'p-petrol-95'
const P_98 = 'p-petrol-98'
const P_DIESEL = 'p-diesel-eu'
const P_LUBE = 'p-lube-10w'
const BASE_PORT = 'b-port'
const BASE_DEPOT = 'b-depot'
const WH_1 = 'wh-1'
const WH_2 = 'wh-2'
const WH_3 = 'wh-3'
const S_1 = 's-1'
const S_2 = 's-2'
const S_3 = 's-3'
const S_4 = 's-4'

function mkProduct(id: string, groupId: string, name: string): any {
  return { id, productGroupId: groupId, commonName: name, longName: name, manufacturerId: null, isComponent: false, addIdentification: null }
}
function mkGroup(id: string, typeId: string, name: string): any {
  return { id, productTypeId: typeId, commonName: name, longName: name }
}
function mkType(id: string, name: string): any {
  return { id, commonName: name, longName: name }
}
function mkBase(id: string, name: string): any {
  return { id, commonName: name, longName: name }
}
function mkWarehouse(id: string, baseId: string, name: string): any {
  return { id, baseId, commonName: name, longName: name }
}
function mkStorage(id: string, warehouseId: string, name: string): any {
  return { id, warehouseId, commonName: name, capacity: '10000', isTypeSpecific: false, productTypeId: null }
}
function mkEntry(contractorId: string, productId: string, storageId: string, amount: string): any {
  return { id: `le-${productId}-${storageId}`, contractorId, productId, storageId, currentAmount: amount }
}

function makeInput(overrides: Partial<BuilderInput> = {}): BuilderInput {
  return {
    contractorId: CONTRACTOR_A,
    ledgerBalances: [],
    products: [
      mkProduct(P_95, PG_GASOLINE, 'Petrol 95'),
      mkProduct(P_98, PG_GASOLINE, 'Petrol 98'),
      mkProduct(P_DIESEL, PG_DIESEL, 'Diesel EU'),
      mkProduct(P_LUBE, PG_LUBE, 'Lube 10W'),
    ],
    productGroups: [
      mkGroup(PG_GASOLINE, PT_FUEL, 'Gasoline'),
      mkGroup(PG_DIESEL, PT_FUEL, 'Diesel'),
      mkGroup(PG_LUBE, PT_OIL, 'Lubricants'),
    ],
    productTypes: [
      mkType(PT_FUEL, 'Fuel'),
      mkType(PT_OIL, 'Oil'),
    ],
    bases: [
      mkBase(BASE_PORT, 'Port-1'),
      mkBase(BASE_DEPOT, 'Depot-A'),
    ],
    warehouses: [
      mkWarehouse(WH_1, BASE_PORT, 'WH-1'),
      mkWarehouse(WH_2, BASE_PORT, 'WH-2'),
      mkWarehouse(WH_3, BASE_DEPOT, 'WH-3'),
    ],
    storages: [
      mkStorage(S_1, WH_1, 'S-1'),
      mkStorage(S_2, WH_1, 'S-2'),
      mkStorage(S_3, WH_2, 'S-3'),
      mkStorage(S_4, WH_3, 'S-4'),
    ],
    showType: false,
    showBase: false,
    searchQuery: '',
    ...overrides,
  }
}

function collectLeafIds(node: AxisNode): string[] {
  if (node.kind === 'leaf')
    return [node.id]
  return node.children.flatMap(collectLeafIds)
}

describe('buildMatrixVM', () => {
  it('returns empty product axis when ledger is empty but keeps all storages', () => {
    const vm = buildMatrixVM(makeInput({ ledgerBalances: [] }))
    expect(vm.productAxis.root.children.length).toBe(0)
    expect(vm.stats.leafRowCount).toBe(0)
    expect(vm.stats.leafColCount).toBe(4)
    expect(vm.grandTotal).toBe(0)
    expect(vm.stats.orphanCount).toBe(0)
  })

  it('includes a product row and populates totals when one non-zero entry exists', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [mkEntry(CONTRACTOR_A, P_95, S_1, '120')],
    }))
    expect(vm.stats.leafRowCount).toBe(1)
    expect(vm.cell(P_95, S_1)).toBe(120)
    expect(vm.rowTotals.get(P_95)).toBe(120)
    expect(vm.colTotals.get(S_1)).toBe(120)
    expect(vm.grandTotal).toBe(120)
    expect(vm.stats.nonEmptyCellCount).toBe(1)
  })

  it('excludes products that have no non-zero entry for the selected contractor', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_A, P_98, S_2, '0'),
      ],
    }))
    expect(vm.stats.leafRowCount).toBe(1)
    const leafIds = collectLeafIds(vm.productAxis.root)
    expect(leafIds).toContain(P_95)
    expect(leafIds).not.toContain(P_98)
  })

  it('adds the product type level when showType is true', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_A, P_LUBE, S_2, '10'),
      ],
      showType: true,
    }))
    const typeLevels = vm.productAxis.root.children
    expect(typeLevels.every(n => n.kind === 'group' && n.level === 'type')).toBe(true)
  })

  it('omits the product type level when showType is false', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [mkEntry(CONTRACTOR_A, P_95, S_1, '120')],
      showType: false,
    }))
    const topLevels = vm.productAxis.root.children
    expect(topLevels.every(n => n.kind === 'group' && n.level === 'group')).toBe(true)
  })

  it('adds the base level when showBase is true', () => {
    const vm = buildMatrixVM(makeInput({ showBase: true }))
    const baseLevels = vm.storageAxis.root.children
    expect(baseLevels.every(n => n.kind === 'group' && n.level === 'base')).toBe(true)
  })

  it('populates group subtotals for both axes', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_A, P_98, S_1, '45'),
      ],
    }))
    expect(vm.groupSubtotals.get('product')?.get(PG_GASOLINE)).toBe(165)
    expect((vm.groupSubtotals.get('product')?.size ?? 0)).toBeGreaterThan(0)
  })

  it('prunes products whose name does not match the search query', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_A, P_DIESEL, S_1, '300'),
      ],
      searchQuery: 'petrol',
    }))
    const leafIds = collectLeafIds(vm.productAxis.root)
    expect(leafIds).toContain(P_95)
    expect(leafIds).not.toContain(P_DIESEL)
  })

  it('prunes storages whose name does not match the search query', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_A, P_95, S_3, '80'),
      ],
      searchQuery: 'wh-2',
    }))
    const leafIds = collectLeafIds(vm.storageAxis.root)
    expect(leafIds).toContain(S_3)
    expect(leafIds).not.toContain(S_1)
  })

  it('counts orphan ledger entries and excludes them from totals', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, 'nonexistent-product', S_1, '500'),
        mkEntry(CONTRACTOR_A, P_95, 'nonexistent-storage', '300'),
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
      ],
    }))
    expect(vm.stats.orphanCount).toBe(2)
    expect(vm.grandTotal).toBe(120)
  })

  it('excludes ledger entries belonging to a different contractor', () => {
    const vm = buildMatrixVM(makeInput({
      contractorId: CONTRACTOR_A,
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_B, P_95, S_1, '999'),
      ],
    }))
    expect(vm.grandTotal).toBe(120)
    expect(vm.cell(P_95, S_1)).toBe(120)
  })

  it('returns an empty VM when contractorId is null', () => {
    const vm = buildMatrixVM(makeInput({ contractorId: null }))
    expect(vm.stats.leafRowCount).toBe(0)
    expect(vm.grandTotal).toBe(0)
  })

  it('computes per-cell subtotal sums for each group and cross-leaf', () => {
    const vm = buildMatrixVM(makeInput({
      ledgerBalances: [
        mkEntry(CONTRACTOR_A, P_95, S_1, '120'),
        mkEntry(CONTRACTOR_A, P_98, S_1, '45'),
        mkEntry(CONTRACTOR_A, P_95, S_3, '80'),
      ],
    }))

    expect(vm.cellSubtotal('product', PG_GASOLINE, S_1)).toBe(165)

    expect(vm.cellSubtotal('product', PG_GASOLINE, S_3)).toBe(80)

    expect(vm.cellSubtotal('product', PG_GASOLINE, S_2)).toBeUndefined()

    expect(vm.cellSubtotal('storage', WH_1, P_95)).toBe(120)
  })
})
