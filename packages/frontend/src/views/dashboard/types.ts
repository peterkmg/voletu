// packages/frontend/src/views/dashboard/types.ts

import type { BaseResponse } from '~/generated/types/BaseResponse'
import type { LedgerBalanceResponse } from '~/generated/types/LedgerBalanceResponse'
import type { ProductGroupResponse } from '~/generated/types/ProductGroupResponse'
// Re-exported generated types — use the canonical names from src/generated/*.
// If a generated name differs, update this file and the consumers together.
import type { ProductResponse } from '~/generated/types/ProductResponse'
import type { ProductTypeResponse } from '~/generated/types/ProductTypeResponse'
import type { StorageResponse } from '~/generated/types/StorageResponse'
import type { WarehouseResponse } from '~/generated/types/WarehouseResponse'

export type Uuid = string

export type Orientation = 'products-as-rows' | 'storages-as-rows'

export type AxisKind = 'product' | 'storage'

// Single union covering both axes. Product axis uses 'type' | 'group' | 'product';
// storage axis uses 'base' | 'warehouse' | 'storage'; 'root' is shared.
// Intentionally not split into two narrower unions — the builder always creates
// nodes with the correct level per axis kind, and keeping one union lets the
// axis-building code be symmetric. A mis-set level is a bug, not a type error.
export type AxisLevel
  = | 'root'
    | 'type' | 'group' | 'product'
    | 'base' | 'warehouse' | 'storage'

export interface LeafNode {
  kind: 'leaf'
  level: AxisLevel
  id: Uuid
  label: string
  sortKey: string
}

export interface GroupNode {
  kind: 'group'
  level: AxisLevel
  id: string // synthetic id for synthetic groups (e.g. "ungrouped"), else the entity uuid
  label: string
  sortKey: string
  children: AxisNode[]
}

export type AxisNode = LeafNode | GroupNode

export interface Axis {
  kind: AxisKind
  root: GroupNode
}

export interface MatrixVM {
  productAxis: Axis
  storageAxis: Axis
  // Non-serializable — closes over a private Map in the builder.
  // To persist or transfer a MatrixVM, rebuild from BuilderInput on the receiving side.
  cell: (productLeafId: Uuid, storageLeafId: Uuid) => number | undefined
  rowTotals: Map<Uuid, number> // by product leaf id
  colTotals: Map<Uuid, number> // by storage leaf id
  // Populated for every active group level on both axes (product/storage).
  // When a structure toggle is off (e.g. showType=false), the corresponding
  // level is absent from the axis tree, so no subtotals for that level are added.
  groupSubtotals: Map<AxisKind, Map<string, number>>
  // Per-cell subtotal lookup for subtotal strip rendering.
  // Returns the sum of all leaf values for leaves under `groupId` on `axis`,
  // against the cross-axis leaf `crossLeafId`. Undefined when there are no
  // entries for that combination.
  cellSubtotal: (axis: AxisKind, groupId: string, crossLeafId: Uuid) => number | undefined
  grandTotal: number
  stats: {
    leafRowCount: number
    leafColCount: number
    nonEmptyCellCount: number
    orphanCount: number
  }
}

export interface BuilderInput {
  contractorId: Uuid | null
  ledgerBalances: LedgerBalanceResponse[]
  products: ProductResponse[]
  productGroups: ProductGroupResponse[]
  productTypes: ProductTypeResponse[]
  storages: StorageResponse[]
  warehouses: WarehouseResponse[]
  bases: BaseResponse[]
  showType: boolean
  showBase: boolean
  searchQuery: string
}

export interface DashboardData {
  vm: MatrixVM | null
  contractors: Array<{ id: string, label: string }>
  isLoading: boolean
  isError: boolean
  error: unknown
  hasAnyData: boolean
  refetchAll: () => void
}

export interface SubtotalToggles {
  productGroup: boolean
  productType: boolean
  warehouse: boolean
  base: boolean
}
