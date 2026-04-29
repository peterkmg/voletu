import type { Axis, MatrixVM, SubtotalToggles } from '~/views/dashboard/types'
import { describe, expect, it } from 'vitest'
import { deriveMatrixLayout } from '~/views/dashboard/components/matrix-layout'

const PRODUCT_1 = 'product-1'
const PRODUCT_GROUP_1 = 'product-group-1'
const STORAGE_1 = 'storage-1'
const STORAGE_2 = 'storage-2'
const WAREHOUSE_1 = 'warehouse-1'
const BASE_1 = 'base-1'

const productAxis: Axis = {
  kind: 'product',
  root: {
    kind: 'group',
    level: 'root',
    id: 'product-root',
    label: 'Products',
    sortKey: 'Products',
    children: [
      {
        kind: 'group',
        level: 'group',
        id: PRODUCT_GROUP_1,
        label: 'Product Group 1',
        sortKey: 'Product Group 1',
        children: [
          {
            kind: 'leaf',
            level: 'product',
            id: PRODUCT_1,
            label: 'Product 1',
            sortKey: 'Product 1',
          },
        ],
      },
    ],
  },
}

const storageAxis: Axis = {
  kind: 'storage',
  root: {
    kind: 'group',
    level: 'root',
    id: 'storage-root',
    label: 'Storages',
    sortKey: 'Storages',
    children: [
      {
        kind: 'group',
        level: 'base',
        id: BASE_1,
        label: 'Base 1',
        sortKey: 'Base 1',
        children: [
          {
            kind: 'group',
            level: 'warehouse',
            id: WAREHOUSE_1,
            label: 'Warehouse 1',
            sortKey: 'Warehouse 1',
            children: [
              {
                kind: 'leaf',
                level: 'storage',
                id: STORAGE_1,
                label: 'Storage 1',
                sortKey: 'Storage 1',
              },
              {
                kind: 'leaf',
                level: 'storage',
                id: STORAGE_2,
                label: 'Storage 2',
                sortKey: 'Storage 2',
              },
            ],
          },
        ],
      },
    ],
  },
}

const storageAxisWithoutBase: Axis = {
  kind: 'storage',
  root: {
    kind: 'group',
    level: 'root',
    id: 'storage-root',
    label: 'Storages',
    sortKey: 'Storages',
    children: [
      {
        kind: 'group',
        level: 'warehouse',
        id: WAREHOUSE_1,
        label: 'Warehouse 1',
        sortKey: 'Warehouse 1',
        children: [
          {
            kind: 'leaf',
            level: 'storage',
            id: STORAGE_1,
            label: 'Storage 1',
            sortKey: 'Storage 1',
          },
          {
            kind: 'leaf',
            level: 'storage',
            id: STORAGE_2,
            label: 'Storage 2',
            sortKey: 'Storage 2',
          },
        ],
      },
    ],
  },
}

const vm: MatrixVM = {
  productAxis,
  storageAxis,
  cell: () => undefined,
  rowTotals: new Map([[PRODUCT_1, 0]]),
  colTotals: new Map([[STORAGE_1, 0], [STORAGE_2, 0]]),
  groupSubtotals: new Map(),
  cellSubtotal: () => undefined,
  grandTotal: 0,
  stats: {
    leafRowCount: 1,
    leafColCount: 2,
    nonEmptyCellCount: 0,
    orphanCount: 0,
  },
}

const vmWithoutStorageTop: MatrixVM = {
  ...vm,
  storageAxis: storageAxisWithoutBase,
}

const subtotals: SubtotalToggles = {
  productGroup: false,
  productType: false,
  warehouse: true,
  base: true,
}

describe('deriveMatrixLayout', () => {
  it('interleaves column subtotal slots and precomputes group spans and labels', () => {
    const layout = deriveMatrixLayout(vm, 'products-as-rows', subtotals)

    expect(layout.colSlots.map(slot => slot.key)).toEqual([
      'leaf:storage-1',
      'leaf:storage-2',
      'mid-subtotal:warehouse-1',
      'top-subtotal:base-1',
    ])
    expect(layout.topGroupSpans.get(BASE_1)).toBe(4)
    expect(layout.midGroupSpans.get(WAREHOUSE_1)).toBe(3)
    expect(layout.colTopGroups[0]?.leafSpan).toBe(2)
    expect(layout.colMidGroups[0]?.leafSpan).toBe(2)
    expect(layout.leafLabels.storage.get(STORAGE_1)).toBe('Storage 1')
    expect(layout.leafLabels.product.get(PRODUCT_1)).toBe('Product 1')
    expect(layout.storageWarehouseLabels.get(STORAGE_1)).toBe('Warehouse 1')
    expect(layout.totalCols).toBe(6)
    expect(layout.hasColTop).toBe(true)
  })

  it('derives product columns when storages are rows', () => {
    const layout = deriveMatrixLayout(vm, 'storages-as-rows', {
      ...subtotals,
      productGroup: true,
    })

    expect(layout.rowAxisKind).toBe('storage')
    expect(layout.colAxisKind).toBe('product')
    expect(layout.colSlots.map(slot => slot.key)).toEqual([
      'leaf:product-1',
      'mid-subtotal:product-group-1',
    ])
    expect(layout.midGroupSpans.get(PRODUCT_GROUP_1)).toBe(2)
    expect(layout.leafLabels.product.get(PRODUCT_1)).toBe('Product 1')
    expect(layout.totalCols).toBe(4)
    expect(layout.hasColTop).toBe(false)
  })

  it('handles storage columns without a base top level', () => {
    const layout = deriveMatrixLayout(vmWithoutStorageTop, 'products-as-rows', subtotals)

    expect(layout.hasColTop).toBe(false)
    expect(layout.colTop).toBeNull()
    expect(layout.colTopGroups).toEqual([])
    expect(layout.colSlots.map(slot => slot.key)).toEqual([
      'leaf:storage-1',
      'leaf:storage-2',
      'mid-subtotal:warehouse-1',
    ])
    expect(layout.midGroupSpans.get(WAREHOUSE_1)).toBe(3)
    expect(layout.storageWarehouseLabels.get(STORAGE_2)).toBe('Warehouse 1')
  })
})
