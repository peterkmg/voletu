import type { BaseResponse } from '~/generated/types/BaseResponse'
import type { LedgerBalanceResponse } from '~/generated/types/LedgerBalanceResponse'
import type { ProductGroupResponse } from '~/generated/types/ProductGroupResponse'

import type { ProductResponse } from '~/generated/types/ProductResponse'
import type { ProductTypeResponse } from '~/generated/types/ProductTypeResponse'
import type { StorageResponse } from '~/generated/types/StorageResponse'
import type { WarehouseResponse } from '~/generated/types/WarehouseResponse'

export type Uuid = string

export type Orientation = 'products-as-rows' | 'storages-as-rows'

export type AxisKind = 'product' | 'storage'

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
  id: string
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

  cell: (productLeafId: Uuid, storageLeafId: Uuid) => number | undefined
  rowTotals: Map<Uuid, number>
  colTotals: Map<Uuid, number>

  groupSubtotals: Map<AxisKind, Map<string, number>>

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
